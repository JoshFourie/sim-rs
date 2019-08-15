use crate::test::zombie::agents;
use agents::{GetPackage, package};

use crate::{context, message, utils};
use context::grid;
use utils::Cell;
use message::MessageFlush;

use rand::Rng;
use log::*;

pub struct Selection<'a> {
    package: &'a package::Package 
}

impl<'a> Selection<'a> {
    pub fn new(package: &'a package::Package) -> Self {
        Self { package }
    }

    pub fn select_tile_from_grid(self) -> Option<context::GridPosition<package::Package>> {
        let agent: _ = self.package;
        let own_kind: _ = agent.as_kind();

        let tile: Option<Cell<grid::Point<_>>> = agent.as_tile()
            .borrow()
            .get_connections()
            .unwrap()
            .iter()
            .max_by(|x,y| {
                let evaluated_x: isize = Evaluation::new(x,own_kind, &self.package).evaluate_tile();
                let evaluated_y: isize = Evaluation::new(y,own_kind, &self.package).evaluate_tile();
                evaluated_x.cmp(&evaluated_y)
            }).cloned();

        self.log(&tile);

        tile.map(|inner| {
            let grid: _ = self.package.as_grid();
            context::GridPosition::new(inner, grid)
        })
    }

    fn log(&self, tile: &Option<Cell<grid::Point<package::Package>>>) {
        info!("{} selected {:?} for their move.", self.package.get_id(), tile)
    }
}

struct Evaluation<'a> {
    target: &'a Cell<grid::Point<package::Package>>,
    kind: &'a agents::Kind,
    score: isize,
    agent: &'a package::Package
}

impl<'a> Evaluation<'a> {
    pub fn new(
        target: &'a Cell<grid::Point<package::Package>>, 
        kind: &'a agents::Kind,
        agent: &'a package::Package
    ) -> Self {
        Self { 
            target,
            kind,
            agent,
            score: 0
        }
    }

    pub fn evaluate_tile(self) -> isize {
        self.kind()
            .proximity()
            .score
    }

    fn kind(mut self) -> Self {
        if let grid::PointState::Occupied(occupier) = self.target.borrow().state() {
            if occupier.as_kind().is(self.kind) {
                self.score += rand::thread_rng().gen_range(10,15)
            } else if self.kind.is(&agents::Kind::Human) {
                let observer: _ = self.agent.as_observer();
                if observer.borrow().human_count > observer.borrow().zombie_count/4 {
                    self.score += 100;
                } else {
                   self.score -= 20   
                }
            } else {
                self.score += 100
            }
        };
        self
    }

    fn proximity(mut self) -> Self {
        let oracle: utils::Cell<context::grid::Grid<_>> = self.agent.as_grid();
        let target: _ = self.target.borrow();
        let agent_tile: _ = self.agent.as_tile();
        let distance: _ = oracle.borrow()
            .toroidal_distance_between(&target, &agent_tile.borrow());

        for (_, msg) in self.agent
            .get_package()
            .as_messenger()
            .agent_inbox
            .borrow_mut()
            .flush_messages()
        {
            if let agents::Message::Report(report) = msg {
                self = DistanceEvaluation::new(report, distance, self).sort();
            }
        }
        self
    } 

}

struct DistanceEvaluation<'a> {
    report: agents::Report,
    benchmark_distance: usize,
    evaluation: Evaluation<'a>
}

impl<'a> DistanceEvaluation<'a> {
    fn new(report: agents::Report, benchmark_distance: usize, evaluation: Evaluation<'a>) -> Self {
        Self {
            report, 
            benchmark_distance, 
            evaluation
        }
    }

    fn sort(self) -> Evaluation<'a> {
        if self.report.is(&agents::ReportKind::Allies) {
            self.ally().evaluation
        } else {
            self.enemy().evaluation
        }
    }

    fn enemy(self) -> Self {
        let agent: &package::Package = self.evaluation.agent;
        if agent.as_kind().is(&agents::Kind::Human) {
            let observer: _ = agent.as_observer();
            if observer.borrow().human_count > observer.borrow().zombie_count/4 {
                self.report_modifier(
                    [std::ops::AddAssign::add_assign,std::ops::AddAssign::add_assign],
                    [30, 10]
                )      
            } else {
                self.report_modifier(
                    [std::ops::SubAssign::sub_assign,std::ops::SubAssign::sub_assign],
                    [20, 10]
                )   
            }
        } else {
            self.report_modifier(
                [std::ops::AddAssign::add_assign,std::ops::AddAssign::add_assign],
                [30, 10]
            )      
        }
    }

    fn ally(self) -> Self {       
        self.report_modifier(
            [std::ops::AddAssign::add_assign,std::ops::AddAssign::add_assign],
            [20, 5]
        )      
    }

    fn report_modifier<F>(mut self, modifiers: [F; 2], modifier_values: [isize; 2]) -> Self
    where
        F: Fn(&mut isize, isize)
    {
        let target: _ = &self.evaluation
            .target
            .borrow();
        let others: &Vec<grid::PointIndex> = self.report.as_inner();

        // measures whether the selected tile will
        // bring the agent closer to another agent
        // whose location was revealed in the report
        // and applies the specified effect
        for other in others.iter() {
            let oracle: _ = self.evaluation.agent.as_grid();
            let active_oracle: _ = oracle.borrow();
            let reported_position: _ = {
                let (row,col): _ = other.as_dimensions();
                active_oracle[row][col].borrow()
            };
            let distance: _ = active_oracle.toroidal_distance_between(&target, &reported_position);

            if &target.get_idx() == other {
                modifiers[0](&mut self.evaluation.score, modifier_values[0]);
            } else if distance < self.benchmark_distance {
                modifiers[1](&mut self.evaluation.score, modifier_values[1]);
            }
        } 
        self
    }
}
