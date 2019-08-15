use crate::{utils,message,context};
use std::{fmt,cell};

#[derive(Clone)]
pub struct Agent<I,M,K,T> {
    comm: message::MessageInterface<I,M>,
    position: utils::Cell<context::GridPosition<T>>,
    kind: K,
    id: I,
}

impl<I,M,K,T> Agent<I,M,K,T> {
    pub fn new(comm: message::MessageInterface<I,M>, position: utils::Cell<context::GridPosition<T>>, kind: K, id: I, ) -> Self {
        Self {comm, position, kind, id}
    }
    
    #[inline]
    pub fn as_tile(&self) -> cell::Ref<context::GridPosition<T>> {
        self.position.borrow()
    }

    #[inline]
    pub fn as_grid(&self) -> utils::Cell<context::grid::Grid<T>> {
        self.position.borrow().get_grid()
    }

    #[inline]
    pub fn swap_stored_position_with(&self, new: context::GridPosition<T>) {
        *self.position.borrow_mut() = new;
    }

    #[inline]
    pub fn as_messenger(&self) -> &message::MessageInterface<I,M> {
        &self.comm
    }

    #[inline]
    pub fn as_kind(&self) -> &K {
        &self.kind
    }

    #[inline]
    pub fn get_id(&self) -> &I {
        &self.id
    }    
}

impl<I,M,K,T> fmt::Debug for Agent<I,M,K,T> 
where
    I: fmt::Debug,
    M: fmt::Debug,
    K: fmt::Debug,
    T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Id: {:?} \nKind: {:?} \nPosition: {:?}", 
            self.get_id(),
            self.as_kind(),
            self.as_tile()
        )
    }
}
