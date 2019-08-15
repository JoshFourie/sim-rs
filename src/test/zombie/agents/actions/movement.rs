use crate::test::zombie::{agents};
use agents::{Agent, package};

use crate::context;

use log::*;

pub struct Movement<'a> {
    agent: &'a dyn Agent,
    new: context::GridPosition<package::Package>
}

impl<'a> Movement<'a> {
    pub fn new(agent: &'a dyn Agent, new: context::GridPosition<package::Package>) -> Self {
        Self {agent, new}
    }

    pub fn step(self) {

        info!("{} is moving onto {:?} from {:?}", self.agent.get_id(),self.new, self.agent.get_package().as_tile());

        if self.new
            .borrow()
            .state()
            .is_empty()
        {         
            let package: &package::Package = self.agent.get_package();
            package.as_tile()
                .borrow_mut()
                .move_inner_into(&mut self.new.borrow_mut());
            package.swap_stored_position_with(self.new);
        } else {
            unimplemented!()
        }
    }
}