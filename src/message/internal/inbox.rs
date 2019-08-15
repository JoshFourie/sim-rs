use std::{fmt,collections,hash,cell,rc};

use crate::utils;

use utils::Cell;

use super::{Inbox, MessageFlush, MessageQueue};

pub struct AgentInbox<I,M> {
    received_messages: collections::HashMap<I,M>
}

impl<I,M> AgentInbox<I,M> 
where
    I: Eq + hash::Hash
{
    pub fn into_cell(self) -> Cell<Self> { rc::Rc::new(cell::RefCell::new(self)) }

    pub fn get_messages<'a>(&'a self) -> &'a collections::HashMap<I, M> { &self.received_messages } 
}

impl<I,M> Inbox<I,M> for AgentInbox<I,M>
where
    I: Eq + hash::Hash,
{
    fn read_msg<'a>(&'a self, id: &'a I) -> Option<&'a M> {
        self.received_messages.get(id)
    }
}

impl<I,M> MessageQueue<I,M> for AgentInbox<I,M>
where
    I: Eq + hash::Hash 
{
    fn push(&mut self, sender_id: I, message: M) {
        self.received_messages
            .insert(sender_id, message);
    }

    fn pop(&mut self) -> Option<(&I, &M)> {
        self.received_messages
            .iter()
            .last()
    }   
}

impl<'a,I:'a,M:'a> MessageFlush<'a,I,M> for AgentInbox<I,M>
{
    type Drain = collections::hash_map::Drain<'a,I,M>;

    fn flush_messages(&'a mut self) -> Self::Drain {
        self.received_messages.drain()  
    }
}

impl<I,M> Default for AgentInbox<I,M>
where
    I: Eq + hash::Hash 
{
    fn default() -> Self {
        Self { received_messages: collections::HashMap::new() }
    }
}

impl<I,M> fmt::Debug for AgentInbox<I,M> 
where
    I: fmt::Debug,
    M: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let received_messages: Vec<_> = self.received_messages
            .iter()
            .collect();
        write!(f, "Inbox: {:?}", received_messages)
    }
}
