mod world;
mod agents;
mod event;

use crate::environment;
use environment::Environment;

use log::*;

#[test]
fn zombie_test_main()
{
    let mut env: Environment<_, Box<dyn agents::Agent>>  = world::EnvironmentFactory::spawn();
    let mut turns_taken: usize = 0;

    warn!("Simulating 40,000 agents...");
    'master: loop {
        for (_, packages) in env.iter_mut() {
            for package in packages.iter_mut() {

                info!("\n{:?}", package);
                if package.is_alive() {
                    package.act();
                };  

                turns_taken += 1;

                let observer: _ = package.get_package().as_observer().borrow();
                let zombie_count = observer.zombie_count;
                let human_count= observer.human_count;

                if zombie_count < 1 {
                    warn!("It took {} turns to complete the simulation. The Humans won.", turns_taken);
                    break 'master
                } else if human_count < 1 {
                    warn!("It took {} turns to complet the simulation. The Zombies won.", turns_taken);
                    break 'master
                } else {
                    info!("{} Zombies are still in play.", zombie_count);
                    info!("{} Humans are still in play.", human_count);
                }
            }
        }
    }
}

// Todo: 
// [ ] Consensus routine
// [ ] logger
// [ ] clean up
