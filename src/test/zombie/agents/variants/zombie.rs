use crate::{context};

use crate::test::zombie::agents;
use agents::{package, actions, GetPackage};
use actions::interaction;
use interaction::skirmish;

use std::{fmt};

pub struct Zombie {
    package: package::Package
}

impl agents::Agent for Zombie {
    fn get_id(&self) -> usize { self.package.get_id() }

    fn get_kind(&self) -> &agents::Kind { &self.get_package().as_kind() }
    
    fn kind_count(&self) -> isize { self.package.as_observer().borrow().zombie_count }
}

impl agents::Action for Zombie {
    fn act(&mut self) {
        let selected_tile: _ = self.select_tile().expect("could not select a tile");
        if selected_tile.borrow().state().is_empty() {
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

impl agents::DeadOrAlive for Zombie { }

impl agents::GetPackage for Zombie {
    fn get_package(&self) -> &package::Package {
        &self.package
    }
}

impl Zombie {
    pub fn new(package: package::Package) -> Self {
        Self {
            package
        }
    }

    fn interact_with(&mut self, other: &package::Package) {
        let package: &package::Package = &self.package;
        if !package.as_kind().is(other.as_kind()) {
            skirmish::Skirmish::new(package, &other).fight()
        }
    }

    fn select_tile(&self) -> Option<context::GridPosition<package::Package>> {
        actions::tile::Selection::new(&self.package).select_tile_from_grid()
    }

    fn walk_to(&mut self, new: context::GridPosition<package::Package>) {
        actions::movement::Movement::new(self, new).step()
    }
}

impl fmt::Debug for Zombie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.package.fmt(f)
    }
}
