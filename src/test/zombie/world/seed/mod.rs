use crate::{utils,environment,context,message};
use utils::Cell;
use context::grid;

use crate::test::zombie::{agents};
use agents::{package, Agent, Kind, Message, variants};
use variants::{human, zombie};

use log::*;

pub mod feature;
pub mod spawn;

pub struct EnvironmentSeed {
    spawn: spawn::SpawnSeed<usize>,    
    feature: feature::FeatureSeed<usize,Message,package::Package>
}

impl environment::Seed for EnvironmentSeed 
{
    type Output = Box<dyn Agent>;
    type Kind = Kind;

    fn seed(&mut self) -> Self::Output {
        let id: usize = self.spawn.new_id();
        let kind: Kind = self.kind();

        let comm: message::MessageInterface<_,_> = self.feature.new_communicator(id);
        let position: context::GridPosition<_> = self.feature.new_position();
        let observer: Cell<package::Observer> = self.feature.new_observer(&kind);

        let package: _ = package::Package::new(comm, position, kind.clone(),id, observer);

        info!("\nSeeding agent: {}", id);
        EnvironmentSeed::seed_agent(kind, package)
    }

    fn kind(&self) -> Kind {
        self.spawn
            .kind
            .clone()
    }

    fn quantity(&self) -> std::ops::Range<usize> {
        0..self.spawn.quantity
    }
}

impl EnvironmentSeed 
{
    pub(super) fn new(spawn: spawn::SpawnSeed<usize>, feature: feature::FeatureSeed<usize,Message,package::Package>) -> Self {
        Self { spawn, feature }
    }

    pub(super) fn into_box(self) -> Box<Self> { Box::new(self) }

    fn seed_agent(kind: Kind, package: package::Package) -> Box<dyn Agent> {
        match kind {
            Kind::Human => Box::new(human::Human::new(package)),
            Kind::Zombie => Box::new(zombie::Zombie::new(package))
        }
    }
}
