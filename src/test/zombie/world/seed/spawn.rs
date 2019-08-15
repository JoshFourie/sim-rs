use crate::test::zombie::agents;
use agents::Kind;

pub struct SpawnSeed<I> {
    pub count: I,
    pub kind: Kind,
    pub quantity: usize,
}

impl<I: Clone+Default> SpawnSeed<I> 
where
    I: std::ops::AddAssign<usize>
{
    pub fn new(kind: Kind, quantity: usize) -> Self {
        Self {
            count: Default::default(),
            quantity,
            kind,
        }
    }
    
    pub fn new_id(&mut self) -> I {
        let id: I = self.count.clone();
        self.count += 1;
        id
    }
}
