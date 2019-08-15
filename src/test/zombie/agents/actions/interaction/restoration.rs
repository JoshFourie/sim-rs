use crate::test::zombie;
use zombie::{agents,event};
use agents::package;

use log::*;

use std::ops;
use ops::{Add, AddAssign};

pub struct Restoration<'a> {
    giver: &'a package::Package,
    receiver: &'a package::Package
}

impl<'a> Restoration<'a> {
    pub fn new(giver: &'a package::Package, receiver: &'a package::Package) -> Self {
        Self{ giver, receiver }
    }

    pub fn give_health(self) {
        let mut giver_health: _  = self.giver 
            .as_statistics()
            .health
            .borrow_mut();
        let gifted_health: isize = *giver_health/10;
        
        if *giver_health-gifted_health > *giver_health/5 {
            *giver_health -= gifted_health;            
            self.receiver  
                .as_statistics()
                .health
                .borrow_mut()
                .add_assign(gifted_health);

            Restoration::log(self.giver.get_id(), self.receiver.get_id(), gifted_health)
        }
    }

    fn log(alpha_id: usize, beta_id: usize, health: isize) {
        info!("{:?}", event::Event::Restoration(alpha_id, beta_id, health))
    }
}
