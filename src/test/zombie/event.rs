use std::fmt;

pub enum Event {
    Communication(usize, usize, Box<dyn fmt::Debug>),
    Skirmish(usize, usize, Option<usize>),
    Restoration(usize, usize, isize)
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Event {
    fn as_string(&self) -> String {
        match self {
            Event::Communication(x,y,z) => Event::log_communication(*x,*y,z),
            Event::Skirmish(x,y,z) => Event::log_skirmish(*x,*y,*z),
            Event::Restoration(x,y,z) => Event::log_restoration(*x,*y,*z),
            _ => unimplemented!()
        }
    }

    fn log_communication(alpha: usize, beta: usize, msg: &dyn fmt::Debug) -> String {
        format!("{} sent a message to {}: {:?}.", alpha,beta,msg)
    }

    fn log_skirmish(alpha: usize, beta: usize, winner: Option<usize>) -> String {
        match winner {
            Some(inner) => format!("{} skirmished with {}: {:?} was victorious.", alpha, beta, inner),
            None => format!("{} skirmished with {}: {:?} was victorious.", alpha, beta, winner)
        }        
    }

    fn log_restoration(alpha: usize, beta: usize, health: isize) -> String {
        format!("{} gave {} health: {}.", alpha,beta,health)
    }
}