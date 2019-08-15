pub mod package;
pub mod actions;
pub mod variants;
pub mod message;

use std::fmt;
use log::*;

pub use message::*;

pub trait Agent: fmt::Debug 
    + Action 
    + DeadOrAlive
{
    fn get_id(&self) -> usize;

    fn get_kind(&self) -> &Kind;

    fn kind_count(&self) -> isize;
}

pub trait Action {
    fn act(&mut self);
}

pub trait DeadOrAlive: GetPackage 
{
    fn is_alive(&self) -> bool {
        self.get_package().as_life_switch().is_alive()
    }

    fn set_dead(&self) {
        info!("Removing Agent from Play.");
        let package: _ = self.get_package();

        if package.as_kind().is(&Kind::Human) {
            let mut observer: _ = package.as_observer().borrow_mut();
            observer.adjust(-1, &Kind::Human);
            // variants::zombie::Zombie::new(package);
            
        } else {
            if self.is_alive() {
                package.as_life_switch().set_dead()
            }
            package.as_tile()
                .borrow_mut()
                .replace(crate::context::grid::PointState::Empty);
            
            package.as_observer()
                .borrow_mut()
                .adjust(-1, &self.get_package().as_kind());
        }

    }
}

pub trait GetPackage {
    fn get_package(&self) -> &package::Package;
}


#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Kind {
    Human,
    Zombie
} 

impl Kind {
    pub fn is(&self, target: &Self) -> bool {
        self == target
    }
}
