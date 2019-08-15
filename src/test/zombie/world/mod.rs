use crate::{environment,message,context};

use environment::Population;
use context::grid;

mod seed;
use seed::{spawn,feature};

use super::{agents};
use agents::{Kind, Agent};

use simplelog;
use log::*;
use std::fs;

pub struct EnvironmentFactory;

impl EnvironmentFactory {

    pub fn spawn() -> environment::Environment<Kind,Box<dyn Agent>> {
        
        Self::build_logger();
        
        let config: _ = Self::build_config();

        info!("Populating environment...");
        let mut environment: _ = environment::Environment::new(config)
            .populate()
            .expect("could not generate populated environment");

        Self::occupy_positions(&mut environment);

        environment
    }

    fn build_logger() {
        simplelog::CombinedLogger::init(
            vec![
                simplelog::TermLogger::new(
                    simplelog::LevelFilter::Warn, 
                    simplelog::Config::default(), 
                    simplelog::TerminalMode::Mixed
                ).unwrap(),
                simplelog::WriteLogger::new(
                    simplelog::LevelFilter::Warn, 
                    simplelog::Config::default(), 
                    fs::File::create("./src/test/zombie/events.log").unwrap()
                )
            ]
        ).unwrap();
    }

    fn build_config() -> environment::Configuration<Kind,Box<dyn Agent>> {
        let feature_seed: feature::FeatureSeed<_,_,_> = {
            let addresses: _ = message::AddressCollection::default().into_cell();
            let message_seed: _ = feature::MessageSeed::new(addresses);
            
            info!("Spawning Grid...");
            let grid: _ = context::grid::Grid::new(1000,1000).into_cell();
            let grid_seed: _ = feature::GridSeed::new(grid);

            feature::FeatureSeed::new(message_seed, grid_seed)
        };

        let human_spawn_seed: _ = spawn::SpawnSeed::new(Kind::Human, 20000);
        let zombie_spawn_seed: _ = spawn::SpawnSeed::new(Kind::Zombie, 20000);

        info!("Mapping Population...");
        let population_map: _ = environment::Demographics::new(vec![

            seed::EnvironmentSeed::new(
                human_spawn_seed, feature_seed.clone()
            ).into_box(),

            seed::EnvironmentSeed::new(
                zombie_spawn_seed, feature_seed.clone()
            ).into_box()
        ]);

        let mut config: environment::Configuration<Kind,Box<dyn Agent>> = Default::default();
        config.insert_dummy_population_field(population_map);
        
        config
    }

    fn occupy_positions(environment: &mut environment::Environment<Kind,Box<dyn Agent>>) {
        for (_, agents) in environment.iter_mut() {
            for agent in agents.iter_mut() {
                let agent_package: _ = agent.get_package();
                agent_package.as_tile()
                    .borrow_mut()
                    .replace(grid::PointState::Occupied(agent_package.clone()));
                info!("SpawnPosition: {:?}", agent_package.as_tile().borrow())
            }      
        }
    }
}
