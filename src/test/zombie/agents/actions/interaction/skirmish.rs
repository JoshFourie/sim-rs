use crate::test::zombie;
use zombie::agents;
use agents::package;
use agents::{DeadOrAlive};

use crate::context::grid;

use log::*;

use std::ops::{Div, SubAssign};

pub struct Skirmish<'a> {
    lhs: &'a package::Package,
    rhs: &'a package::Package
} 

impl<'a> Skirmish<'a> {

    pub fn new(lhs: &'a package::Package, rhs: &'a package::Package) -> Self {
        Self{ lhs,rhs }
    }

    pub fn fight(self) {
        let alpha: &package::Package = self.lhs;
        let beta: &package::Package = self.rhs;

        Skirmish::calculate_and_reduce(alpha, beta);

        if *beta.as_statistics().health.borrow() < 1 {
            info!("{} has been killed.", beta.get_id());
            beta.set_dead();
        } else {
            Skirmish::calculate_and_reduce(beta, alpha);
            if *alpha.as_statistics().health.borrow() < 1 {
                info!("{} has been killed.", alpha.get_id());
                alpha.set_dead()
            }
        }
        Skirmish::log(alpha.get_id(), beta.get_id())
    }

    fn calculate_and_reduce(lhs: &package::Package, rhs: &package::Package) {
        let lhs_reduction: isize = Skirmish::calculate_health_reduction(lhs,rhs);
        Skirmish::reduce_health(lhs, lhs_reduction);
        info!("{} lost {} health and now has {:?} health.", lhs.get_id(), lhs_reduction, lhs.as_statistics().health);
    }

    fn calculate_health_reduction(lhs: &package::Package, rhs: &package::Package) -> isize {
        let lhs_health: _ = lhs.as_statistics()
            .health
            .borrow()
            .abs();
        let mut rhs_strength: _ = rhs.as_statistics()
            .strength
            .borrow()
            .abs();

        for tile in rhs.as_tile()
            .borrow()
            .get_connections()
            .expect("expected connections on tile")
            .iter()
        {
            if let grid::PointState::Occupied(other) = tile.borrow().state() {
                if other.as_kind().is(lhs.as_kind()) {
                    rhs_strength += other.as_statistics().strength.borrow().div(2);
                }
            }
        }
        (lhs_health - rhs_strength).abs()
    }

    fn reduce_health(target: &package::Package, reduction: isize) {
        target.as_statistics()
            .health
            .borrow_mut()
            .sub_assign(reduction)
    }

    fn log(alpha_id: usize, beta_id: usize) { info!("{} skirmished with {}.", alpha_id, beta_id) }
}
