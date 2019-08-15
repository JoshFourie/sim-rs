use crate::test::zombie::{agents};
use agents::{package};

use crate::{message, context};
use context::grid;
use message::{MessageQueue};

use log::*;

pub struct Communication<'a> {
    rc_id: usize,
    package: &'a package::Package,
    loaded_messages: Vec<agents::Message>
}

impl<'a> Communication<'a> {
    pub fn new(rc_id: usize, package: &'a package::Package) -> Self {
        Self { 
            rc_id, 
            package,
            loaded_messages: Vec::new()
        }
    }

    pub fn find_enemies(self) -> Self {
        let trigger: _ = |agent: &package::Package, other: &package::Package| -> bool {
            !agent.as_kind().is(other.as_kind())
        };

        self.report(trigger, agents::ReportKind::Enemies)
    }

    pub fn find_allies(self) -> Self {
        let trigger: _ = |agent: &package::Package, other: &package::Package| -> bool {
            agent.as_kind().is(other.as_kind())
        };
        self.report(trigger, agents::ReportKind::Allies)
    }

    fn report<F>(mut self, trigger: F, report_kind: agents::ReportKind) -> Self 
    where
        F: Fn(&package::Package,&package::Package) -> bool
    {
        let agent: _ = &self.package;
        let reports: Vec<grid::PointIndex> = agent.as_tile()
            .borrow()
            .get_connections()
            .expect("no connections on tile")
            .iter()
            .map(|connection| {
                connection.borrow()
                    .get_connections()
                    .expect("no connections on tile")
                    .iter()
                    .filter_map(|tile| {
                        let tile_idx: grid::PointIndex = tile.borrow().get_idx();
                        if agent.as_tile().borrow().get_idx() == tile_idx {
                            None
                        } else if let grid::PointState::Occupied(other) = tile.borrow().state() {
                            if trigger(agent, other) {
                                Some(tile_idx)
                            } else { 
                                None
                            }
                        } else { None }
                    }).collect::<Vec<_>>()
            }).flatten().collect();

        if !reports.is_empty() {
            let report_inner: _ = agents::Report::new(report_kind, reports);
            self.loaded_messages.push(agents::Message::Report(report_inner))
        }
        self
    }

    pub fn prepare(self) {
        info!("{} prepared a message for {}: {:?}", self.package.get_id(), self.rc_id, self.loaded_messages);
        for msg in self.loaded_messages.into_iter() {
            self.package
                .as_messenger()
                .agent_outbox
                .borrow_mut()
                .push(self.rc_id, msg)
        }
    }
}