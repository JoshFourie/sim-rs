use crate::{utils,message,context,agent};
use utils::Cell;

use rand::Rng;

use crate::test::zombie::{agents};
use agents::{Message, Kind};

use std::{fmt, cell};

#[derive(Clone)]
pub struct Package {
    agent: agent::Agent<usize,Message,Kind,Self>,
    stats: Statistics,
    life_switch: LifeSwitch,
    observer: Cell<Observer>
}

impl Package {
    pub fn new(
        comm: message::MessageInterface<usize,Message>, 
        position: context::GridPosition<Self>,
        kind: Kind,
        id: usize,
        observer: Cell<Observer>
    ) -> Self {
        Package {
            stats: Statistics::random(&kind),
            agent: agent::Agent::new(comm,position.into_cell(),kind,id),
            observer,
            life_switch: LifeSwitch::new(),
        }
    }

    #[inline]
    pub fn as_tile(&self) -> cell::Ref<context::GridPosition<Self>> {
        self.agent.as_tile()
    }

    #[inline]
    pub fn as_grid(&self) -> utils::Cell<context::grid::Grid<Self>> {
        self.agent.as_grid()
    }

    #[inline]
    pub fn swap_stored_position_with(&self, new: context::GridPosition<Self>) {
        self.agent.swap_stored_position_with(new)
    }

    #[inline]
    pub fn as_messenger(&self) -> &message::MessageInterface<usize, Message> {
        &self.agent.as_messenger()
    }

    #[inline]
    pub fn as_statistics(&self) -> &Statistics {
        &self.stats
    }

    #[inline]
    pub fn as_kind(&self) -> &Kind {
        &self.agent.as_kind()
    }

    #[inline]
    pub fn get_id(&self) -> usize {
        *self.agent.get_id()
    }

    #[inline]
    pub fn as_life_switch(&self) -> &LifeSwitch {
        &self.life_switch
    }

    #[inline]
    pub fn as_observer(&self) -> &Cell<Observer> {
        &self.observer
    }
}

impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Agent: {} \nKind: {:?} \nPosition: {:?}\nStats: {:?}\nAlive: {}", 
            self.get_id(),
            self.as_kind(),
            self.as_tile(), 
            self.as_statistics(), 
            self.as_life_switch().is_alive()
        )
    }
}

impl agents::DeadOrAlive for Package { }

impl agents::GetPackage for Package { 
    #[inline]
    fn get_package(&self) -> &Self {
        self
    }
}

#[derive(Clone)]
pub struct Statistics {
    pub health: Cell<isize>,
    pub strength: Cell<isize>
}

impl Statistics {
    fn random(kind: &agents::Kind) -> Self {
        match kind {
            Kind::Human => {
                let health: isize = rand::thread_rng().gen_range(1,25);
                let strength: isize = rand::thread_rng().gen_range(1,100);

                Self {
                    health: std::rc::Rc::new(std::cell::RefCell::new(health)),
                    strength: std::rc::Rc::new(std::cell::RefCell::new(strength))
                }
            },
            Kind::Zombie => {
                let health: isize = rand::thread_rng().gen_range(1,50);
                let strength: isize = rand::thread_rng().gen_range(1,25);

                Self {
                    health: std::rc::Rc::new(std::cell::RefCell::new(health)),
                    strength: std::rc::Rc::new(std::cell::RefCell::new(strength))
                }
            }
        }

    }
}

impl fmt::Debug for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Health: {:?}, Strength: {:?}",
            self.health, self.strength
        )
    }
}


#[derive(Clone, Debug)]
pub struct LifeSwitch {
    switch: Cell<bool>
}

impl LifeSwitch {
    pub fn new() -> Self {
        LifeSwitch {
            switch: std::rc::Rc::new(std::cell::RefCell::new(true))
        }
    }

    pub fn is_alive(&self) -> bool {
        *self.switch.borrow()
    }

    pub fn set_dead(&self) {
        self.switch.replace(false);
    }
}

#[derive(Debug)]
pub struct Observer {
    pub human_count: isize,
    pub zombie_count: isize
}

impl Observer {
    pub fn new() -> Self {
        Self { 
            human_count: 0,
            zombie_count: 0
        }
    }

    pub fn adjust(&mut self, count: isize, kind: &agents::Kind) {
        match kind {
            agents::Kind::Human => self.human_count += count,
            agents::Kind::Zombie => self.zombie_count += count
        }
    }

    pub fn into_cell(self) -> Cell<Self> {
        std::rc::Rc::new(std::cell::RefCell::new(self))
    }
}
