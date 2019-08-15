use std::{ops,collections,hash,fmt};

mod population;
mod config;

pub use population::{Population,Seed,Demographics};
pub use config::Configuration;

pub struct Environment<I,T> {
    agents: Storage<I,T>,
    config: config::Configuration<I,T>,
}

impl<I,T> Environment<I,T> 
where
    I: Eq + hash::Hash
{
    pub fn new(config: config::Configuration<I,T>) -> Self {
        Self {
            agents: Storage::new(collections::HashMap::new()),
            config
        }
    }

    pub fn iter_mut(&mut self) -> collections::hash_map::IterMut<I,Vec<T>> {
        self.agents.0.iter_mut()
    }

    pub fn iter(&self) -> collections::hash_map::Iter<I,Vec<T>> {
        self.agents.0.iter()
    }
}

impl<'a,I,T> ops::Index<&'a I> for Environment<I,T> 
where
    I: Eq + hash::Hash
{
    type Output = Vec<T>;
    
    fn index(&self, idx: &I) -> &Self::Output {
        &self.agents.0[idx]
    }
}

impl<I,T> fmt::Debug for Environment<I,T> 
where
    I: fmt::Debug,
    T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.agents)
    }
}

struct Storage<I,T>(pub collections::HashMap<I, Vec<T>>);

impl<I,T> Storage<I,T> {
    fn new(inner: collections::HashMap<I, Vec<T>>) -> Self {
        Self(inner)
    }
}

impl<I,T> fmt::Debug for Storage<I,T>
where
    I: fmt::Debug,
    T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let agents: Vec<_> = self.0.iter().collect();
        write!(f, "{:?}", agents)
    }
}