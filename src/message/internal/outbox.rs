use std::{fmt,collections,hash};

use crate::utils;

use utils::{Cell,sync};

use sync::{GreedyBorrow, GreedyLock};

use super::{inbox,error};

use super::{MessageFlush,Outbox,MessageQueue};

pub struct AgentOutbox<I,M> {
    unsent_messages: collections::HashMap<I,M>,
    pub(super) inbox_addresses: Cell<AddressCollection<I,M>>,
    pub(super) agent_identity: I
}

impl<I,M> AgentOutbox<I,M> 
where
    I: Eq + hash::Hash
{
    pub fn new(id: I, addresses: Cell<AddressCollection<I,M>>) -> Self {
        Self { 
            unsent_messages: collections::HashMap::new(),
            inbox_addresses: addresses,
            agent_identity: id
        }
    }

    #[cfg(test)] pub fn get_unsent_messages<'a>(&'a mut self) -> &'a mut collections::HashMap<I, M> {
        &mut self.unsent_messages
    }

    pub fn into_cell(self) -> Cell<Self> {
        std::rc::Rc::new(std::cell::RefCell::new(self))
    }
}

impl<I: Clone,M> Outbox<I,M> for AgentOutbox<I,M>
where
    I: Eq + hash::Hash
{
    type Output = Result<(), error::MessageError<I,M>>;

    fn try_send(&mut self) -> Self::Output 
    {
        let addresses: &collections::HashMap<_,_> = &self.inbox_addresses
            .await_greedy_borrow()
            .addresses;
        let sender_identity: I = self.agent_identity.clone();

        // TODO: set as environmental variable or configuration file.
        let lock_timer: _ = std::time::Duration::from_millis(100);

        let aborted_messages: Vec<error::AbortedMessage<_,_>> = self
            .unsent_messages
            .drain()
            .filter_map(|(recipient_id,message)| 
            {
                match addresses[&recipient_id].await_greedy_lock_with_timeout(lock_timer) {
                    Ok(mut rc) => {
                        rc.push(sender_identity.clone(), message);
                        None
                    },
                    Err(_) => {
                        let aborted_message: _ = error::AbortedMessage::new(
                            sender_identity.clone(), recipient_id, message
                        );
                        Some(aborted_message)
                    }                    
                }                
            }).collect();

        if aborted_messages.is_empty() {
            Ok(())
        } else {
            let aborted_messages_error: _ = error::MessageErrorKind::AbortedMessages(aborted_messages);
            Err(error::MessageError::from(aborted_messages_error))
        }
    }     
}

impl<I,M> MessageQueue<I,M> for AgentOutbox<I,M> 
where
    I: Eq + hash::Hash
{
    fn push(&mut self, recipient_id: I, message: M) {
        self.unsent_messages
            .insert(recipient_id, message); 
    }

    fn pop(&mut self) -> Option<(&I, &M)> {
        self.unsent_messages
            .iter()
            .last()
    }   
}

impl<'a,I:'a,M:'a> MessageFlush<'a,I,M> for AgentOutbox<I,M>
{
    type Drain = collections::hash_map::Drain<'a,I,M>;

    fn flush_messages(&'a mut self) -> Self::Drain {
        self.unsent_messages.drain()
    }
}

impl<I,M> fmt::Debug for AgentOutbox<I,M> 
where
    I: fmt::Debug,
    M: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unsent_messages: Vec<_> = self.unsent_messages.iter().collect();
        write!(f, "Outbox: {:?}", unsent_messages)
    }
}

pub struct AddressCollection<I,M> {
   addresses: collections::HashMap<I,Cell<inbox::AgentInbox<I,M>>>
}   

impl<I,M> AddressCollection<I,M> 
where   
    I: Eq + hash::Hash
{
    pub fn into_cell(self) -> Cell<Self> {
        std::rc::Rc::new(std::cell::RefCell::new(self))
    }

    pub fn get_addresses(&mut self) -> &mut collections::HashMap<I,Cell<inbox::AgentInbox<I,M>>> {
        &mut self.addresses
    }
}

impl<I,M> Default for AddressCollection<I,M> 
where   
    I: Eq + hash::Hash
{
    fn default() -> Self {
        Self { addresses: collections::HashMap::new() }
    }
} 

impl<I,M> fmt::Debug for AddressCollection<I,M> 
where
    I: fmt::Debug,
    M: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addresses: Vec<_> = self.addresses  
            .iter()
            .collect();
        write!(f, "Addresses: {:?}", addresses)
    }
}
