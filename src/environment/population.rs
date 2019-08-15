use std::hash;

pub trait Population
{
    type Output;

    fn populate(self) -> Self::Output; 
}

pub trait Seed 
{
    type Output;

    type Kind;

    fn seed(&mut self) ->  Self::Output;

    fn kind(&self) -> Self::Kind;

    fn quantity(&self) -> std::ops::Range<usize>;
}

pub trait Configuration 
{
    type Seed;

    fn population(&mut self) -> Option<Self::Seed>;
}

impl<I,T> Population for super::Environment<I,T> 
where
    I: Eq + hash::Hash
{
    type Output = Result<Self, std::option::NoneError>;

    fn populate(mut self) -> Self::Output 
    {
        let config: Demographics<I,T> = self.config.population()?;

        for mut generator in config.into_iter() 
        {            
            let agents: Vec<T> = generator.quantity()
                .map(|_| generator.seed())
                .collect();
                
            self.insert_agents(generator.kind(), agents)            
        }
        
        Ok(self)
    }
}

pub struct Demographics<I,T>(Vec<Box<dyn Seed<Kind=I,Output=T>>>);

impl<I,T> Demographics<I,T> {
    pub fn new(inner: Vec<Box<dyn Seed<Kind=I,Output=T>>>) -> Self {
        Self(inner)
    }
}

impl<I,T> IntoIterator for Demographics<I,T> 
{
    type Item = Box<dyn Seed<Kind=I,Output=T>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<I,T> super::Environment<I,T>
where   
    I: Eq + hash::Hash
{
    fn insert_agents(&mut self, kind: I, agents: Vec<T>) 
    {
        self.agents 
            .0
            .insert(kind, agents);
    }   
}

#[cfg(test)]
mod tests
{
    use super::{Seed,Population};
    use super::super::{Environment, Storage, config};

    #[derive(Default,Clone,Debug,PartialEq,Eq,Hash)]
    struct TestAgent {
        id: usize
    }

    #[derive(Debug,Clone,PartialEq,Eq,Hash)]
    struct TestSeed {
        count: usize,
        kind: TestKind,
        quantity: usize
    }

    impl TestSeed {
        fn new(kind: TestKind, quantity: usize) -> Self {
            TestSeed {
                count: 0,
                kind,
                quantity
            }
        }
    }

    impl Seed for TestSeed
    {
        type Output = TestAgent;
        type Kind = TestKind;

        fn seed(&mut self) -> Self::Output {
            let id = self.count;
            self.count += 1;
            TestAgent { id }
        }    

        fn kind(&self) -> TestKind {
            self.kind.clone()
        }

        fn quantity(&self) -> std::ops::Range<usize> {
            0..self.quantity
        }
    }

    #[derive(Debug,Clone,PartialEq,Eq,Hash)]
    enum TestKind {
        A,
        B,
        C
    }

    fn spawn_populated_environment() -> Environment<TestKind,TestAgent>
    {
        let population_map: super::Demographics<_,_> = super::Demographics(vec![
            Box::new(TestSeed::new(TestKind::A, 10)),
            Box::new(TestSeed::new(TestKind::B, 20)),
            Box::new(TestSeed::new(TestKind::C, 30))
        ]);

        let mut config: _ = config::Configuration::default();
        config.insert_dummy_population_field(population_map);

        Environment {
            agents: Storage(Default::default()),
            config
        }.populate().unwrap()
    }

    #[test]
    fn test_agent_id_increment() 
    {
        let populated_environment: _ = spawn_populated_environment();

        for (_, agents) in populated_environment.agents
            .0
            .iter()
        {
            for (expected_id,agent) in agents.iter()
                .enumerate()
            {
                assert_eq!(agent.id, expected_id)
            }
        }
    }

    #[test]
    fn test_environment_population()
    {
        let populated_environment: _ = spawn_populated_environment();

        let mut expected_agent_storge: _ = std::collections::HashMap::new();
        expected_agent_storge.insert(TestKind::A, Vec::new());
        expected_agent_storge.insert(TestKind::B, Vec::new());
        expected_agent_storge.insert(TestKind::C, Vec::new());

        for i in 0..10 {
            expected_agent_storge.get_mut(&TestKind::A)
                .unwrap()
                .push(TestAgent{ id: i });
        }
        for i in 0..20 {
            expected_agent_storge.get_mut(&TestKind::B)
                .unwrap()
                .push(TestAgent{ id: i });
        }
        for i in 0..30 {
            expected_agent_storge.get_mut(&TestKind::C)
                .unwrap()
                .push(TestAgent{ id: i });
        }

        assert_eq!(populated_environment.agents.0, expected_agent_storge);
    }
}