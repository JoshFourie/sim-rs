use super::{population};

pub struct Configuration<I,T> {
    population: Option<population::Demographics<I,T>>
}

impl<I,T> population::Configuration for Configuration<I,T> 
{
    type Seed = population::Demographics<I,T>;

    fn population(&mut self) -> Option<population::Demographics<I,T>> {
        let mut to_be_config: Option<_> = None;
        std::mem::swap(&mut self.population, &mut to_be_config);
        to_be_config    
    }   
}

#[cfg(test)]
impl<I,T> Default for Configuration<I,T> {
    fn default() -> Self {
        Self {
            population: None
        }
    }
}

#[cfg(test)]
impl<I,T> Configuration<I,T>
{
    pub fn insert_dummy_population_field(&mut self, inner: population::Demographics<I,T>) {
        self.population = Some(inner);
    }
}