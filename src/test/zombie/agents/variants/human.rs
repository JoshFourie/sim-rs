use crate::{message,context};
use message::Outbox;

use crate::test::zombie::agents;
use agents::{package, actions, GetPackage};
use actions::{interaction};
use interaction::{communication,restoration,skirmish};

use std::{fmt};

pub struct Human {
    kind: agents::Kind,
    package: package::Package
}

impl agents::Agent for Human {
    fn get_id(&self) -> usize { self.package.get_id() }

    fn get_kind(&self) -> &agents::Kind { &self.kind }

    fn kind_count(&self) -> isize { self.package.as_observer().borrow().human_count }
}

impl agents::Action for Human {
    fn act(&mut self) 
    {
        let selected_tile: _ = self.select_tile().expect("could not select a tile");
            
        if selected_tile.borrow()
            .state()
            .is_empty() 
        {
            self.walk_to(selected_tile)
        } else {
            let other: _ = &selected_tile.borrow()
                .state()
                .as_inner()
                .unwrap()
                .clone();
            self.interact_with(other);
        }


    }
}

impl agents::DeadOrAlive for Human { }

impl agents::GetPackage for Human {
    fn get_package(&self) -> &package::Package {
        &self.package
    }
}
impl Human {
    pub fn new(package: package::Package) -> Self {
        Self {
            kind: agents::Kind::Human,
            package
        }
    }

    fn interact_with(&mut self, other: &package::Package) {
        let self_package: &package::Package = &self.package;
        if self.kind.is(other.as_kind()) {
            self.communicate_with(other);
            self.give_health_to(other);
        } else {
            skirmish::Skirmish::new(self_package, &other).fight()
        }
    }

    fn select_tile(&self) -> Option<context::GridPosition<package::Package>> {
        actions::tile::Selection::new(&self.package).select_tile_from_grid()
    }

    fn walk_to(&mut self, new: context::GridPosition<package::Package>) {
        actions::movement::Movement::new(self, new).step()
    }

    fn communicate_with(&self, other: &package::Package) {
        let receiver_id: _ = other.get_id();
        let sender: _ = self.get_package();
        communication::Communication::new(receiver_id, sender)
            .find_allies()
            .find_enemies()
            .prepare();

        self.package
            .as_messenger()
            .agent_outbox
            .borrow_mut()
            .try_send()
            .expect("could not send message");
    }

    fn give_health_to(&self, other: &package::Package) {
        restoration::Restoration::new(&self.package, &other).give_health()
    }
}

impl fmt::Debug for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.package.fmt(f)
    }
}
