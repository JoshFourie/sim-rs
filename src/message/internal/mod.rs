use crate::utils;
use utils::{Cell,sync};
use sync::GreedyLock;

use super::{Inbox, Outbox, MessageFlush, MessageQueue, error};

use std::{hash, fmt};

mod inbox;
mod outbox;

pub use outbox::AddressCollection;

#[derive(Clone)] // Clone is acceptable because the fields are both RefCells.
pub struct MessageInterface<I,M> {
    pub agent_inbox: Cell<inbox::AgentInbox<I,M>>,
    pub agent_outbox: Cell<outbox::AgentOutbox<I,M>>
}

impl<I:Clone,M> MessageInterface<I,M> 
where
    I: Eq + hash::Hash
{
    pub fn new(id: I, addresses: Cell<outbox::AddressCollection<I,M>>) -> Self {
        let agent_inbox: _ = inbox::AgentInbox::default().into_cell();
        let agent_outbox: _ = outbox::AgentOutbox::new(id.clone(), addresses).into_cell();

        agent_outbox.borrow()
            .inbox_addresses
            .await_greedy_lock()
            .get_addresses()
            .insert(id, agent_inbox.clone()); 

        Self { agent_inbox, agent_outbox }  
    }
} 

impl<I,M> fmt::Debug for MessageInterface<I,M> 
where
    I: fmt::Debug,
    M: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} \n {:?}", self.agent_inbox, self.agent_outbox)
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    use std::collections;

    use crate::utils::sync::GreedyLock;

    extern crate test;

    struct TestEnvironment {
        agents: Vec<TestAgent>,
        inbox_addresses: Cell<outbox::AddressCollection<TestIdentity,TestMessage>>
    }

    struct TestAgent {
        message_module: MessageInterface<TestIdentity,TestMessage>,
        agent_identity: TestIdentity
    }

    impl TestAgent {
        fn new(id: TestIdentity, addresses: Cell<outbox::AddressCollection<TestIdentity,TestMessage>>) -> Self {
            let message_interface: MessageInterface<_,_> = MessageInterface::new(id,addresses);
            
            Self {
                message_module: message_interface,
                agent_identity: id
            }
        }
    }

    #[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
    enum TestMessage {
        RequestSync,
        Acknowledge,
        Finish,
    }

    #[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
    struct TestIdentity(usize); 

    fn spawn_test_environment(population: usize) -> TestEnvironment
    {   
        let mut agents: Vec<TestAgent> = Vec::new();
        let inbox_addresses: Cell<_> = outbox::AddressCollection::default().into_cell();

        for round in 0..population 
        {
            let new_id: TestIdentity = TestIdentity(round);
            let new_agent: _ = TestAgent::new(new_id, inbox_addresses.clone());
            
            inbox_addresses
                .borrow_mut()
                .get_addresses()
                .insert(
                    new_id, 
                    new_agent.message_module
                        .agent_inbox
                        .clone() 
                );

            agents.push(new_agent)
        }

        TestEnvironment {
            agents,
            inbox_addresses
        }
    }

    #[test]
    fn test_inbox_push() 
    {
        let dummy_identity: _ = TestIdentity(0);
        let dummy_message: _ = TestMessage::RequestSync;

        let mut test_inbox: inbox::AgentInbox<_,_> = inbox::AgentInbox::default();
        test_inbox.push(dummy_identity, dummy_message);

        let mut expected_inbox: collections::HashMap<TestIdentity,TestMessage> = collections::HashMap::new();

        expected_inbox.insert(dummy_identity, dummy_message);

        assert_eq!(test_inbox.get_messages(), &expected_inbox);
    }

    #[ignore="Popping from Inbox returns a random message."]
    #[test]
    fn test_inbox_pop()
    {
        let mut test_inbox: inbox::AgentInbox<_,_> = inbox::AgentInbox::default();
        let population: usize = 10;

        for i in 0..population {
            let identity: _ = TestIdentity(i);
            if i < population-1 {
                test_inbox.push(identity, TestMessage::Acknowledge)
            } else {
                let target_identity: _ = identity;
                let target_message: _ = TestMessage::RequestSync;
                test_inbox.push(target_identity, target_message)
            } 
        }

        let (test_identity,test_message): _ = test_inbox.pop().unwrap();
        let expected_identity: _ = TestIdentity(2);
        let expected_message: _ = TestMessage::RequestSync;

        assert_eq!(test_identity, &expected_identity);
        assert_eq!(test_message, &expected_message);
    }

    #[test]
    fn test_inbox_read()
    {
        let dummy_identity: _ = TestIdentity(0);
        let dummy_message: _ = TestMessage::RequestSync;;

        let mut test_inbox: inbox::AgentInbox<_,_> = inbox::AgentInbox::default();
        test_inbox.push(dummy_identity, dummy_message);

        let test_message: &TestMessage = test_inbox.read_msg(&dummy_identity).unwrap();
        let expected_message: &TestMessage = &TestMessage::RequestSync;

        assert_eq!(test_message, expected_message);   
    }

    #[test]
    fn test_outbox_push() 
    {
        let addresses: _ = spawn_test_environment(2).inbox_addresses;

        let mut test_outbox: outbox::AgentOutbox<_,_> = outbox::AgentOutbox::new(TestIdentity(0), addresses);
        let mut expected_outbox: collections::HashMap<TestIdentity,TestMessage> = collections::HashMap::new();

        test_outbox.push(TestIdentity(1), TestMessage::RequestSync);
        expected_outbox.insert(TestIdentity(1), TestMessage::RequestSync);

        assert_eq!(test_outbox.get_unsent_messages(), &expected_outbox);
    }

    #[ignore="Popping from Outbox returns a random message."]
    #[test]
    fn test_outbox_pop() {
        unimplemented!()
    }

    #[test]
    fn test_try_send_message() 
    {
        let population: usize = 10;
        let mut test_environment: TestEnvironment = spawn_test_environment(population);

        for agent in test_environment.agents
            .iter_mut()
        {
            let sender_id: TestIdentity = agent.agent_identity;
            let agent_outbox: &mut std::cell::RefMut<outbox::AgentOutbox<_,_>> = &mut agent
                .message_module
                .agent_outbox
                .borrow_mut();

            for round in 0..population {
                let recipient_id: TestIdentity = TestIdentity(round);

                if sender_id != recipient_id {
                    agent_outbox.push(recipient_id, TestMessage::RequestSync)
                }
            } 
            agent_outbox.try_send().unwrap()
        }

        for agent in test_environment.agents
            .iter()
        {
            let expected_total_messages: usize = population-1;
            let mut actual_total_messages: usize = 0;   

            for (_,inboxed_message) in agent.message_module
                .agent_inbox
                .borrow()
                .get_messages()
                .iter()
            {
                assert_eq!(inboxed_message, &TestMessage::RequestSync);
                actual_total_messages += 1;
            }

            assert_eq!(actual_total_messages, expected_total_messages)
        }   
    }

    /// # Messages per Iteration
    /// 
    /// 10 agents over 3 rounds -> 210 messages : 125 ns/message.
    /// 
    /// 100 agents over 3 rounds -> 20100 messages : 127.5 ns/message.
    #[bench]
    fn bench_3_round_responsive_communication(benchmark: &mut test::Bencher)
    {
        let population: usize = 10;
        let communication_rounds: usize = 3;
        let mut test_environment: TestEnvironment = spawn_test_environment(population); 

        benchmark.iter(|| {
            for round in 0..communication_rounds {
                for agent in test_environment.agents
                    .iter_mut()
                {
                    if round == 0 {
                        for sub_round in 0..population {
                            agent.message_module
                                .agent_outbox
                                .borrow_mut()
                                .push(TestIdentity(sub_round), TestMessage::RequestSync)
                        } 
                    } else {
                        for (sender_id, inboxed_message) in agent.message_module
                            .agent_inbox
                            .await_greedy_lock()
                            .flush_messages()
                        {
                            let outbound_message: Option<_> = match inboxed_message {
                                TestMessage::RequestSync => Some(TestMessage::Acknowledge),
                                TestMessage::Acknowledge => Some(TestMessage::Finish),
                                TestMessage::Finish => None
                            };

                            if let Some(msg) = outbound_message { 
                                agent.message_module
                                    .agent_outbox
                                    .borrow_mut()
                                    .push(sender_id, msg)
                            }   
                        }    
                    }
                    agent.message_module
                        .agent_outbox
                        .borrow_mut()
                        .try_send()
                        .unwrap();             
                }
            }
        });

        for agent in test_environment.agents
            .iter()
        {
            for (_,message) in agent.message_module
                .agent_inbox
                .borrow_mut()
                .flush_messages()
            {
                assert_eq!(message, TestMessage::Finish)
            }
        }
    }
}