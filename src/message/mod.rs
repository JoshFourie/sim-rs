mod internal;

// mod external;

mod error;

pub use internal::{MessageInterface, AddressCollection};

pub trait Outbox<ID,MSG> 
{
    type Output;

    fn try_send(&mut self) -> Self::Output;
}

pub trait Inbox<ID,MSG> 
{
    fn read_msg<'a>(&'a self, id: &'a ID) -> Option<&'a MSG>;
}

pub trait MessageFlush<'a,ID,MSG> 
{
    type Drain;

    fn flush_messages(&'a mut self) -> Self::Drain;
}

pub trait MessageQueue<ID,MSG> 
{
    fn push(&mut self, id: ID, msg: MSG);

    fn pop(&mut self) -> Option<(&ID, &MSG)>;
}