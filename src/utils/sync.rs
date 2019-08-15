use std::{rc, cell, time, sync};

pub trait GreedyLock 
{
    type Output;

    fn await_greedy_lock<'a:'b,'b>(&'a self) -> cell::RefMut<'b, Self::Output>;

    fn await_greedy_lock_with_timeout<'a:'b,'b>(&'a self, timeout_limit: time::Duration) -> Result<cell::RefMut<'b,Self::Output>, cell::BorrowMutError>;
}

pub trait GreedyBorrow 
{
    type Output;

    fn await_greedy_borrow<'a:'b,'b>(&'a self) -> cell::Ref<'b, Self::Output>;

    fn await_greedy_borrow_with_timeout<'a:'b,'b>(&'a self, timeout_limit: time::Duration) -> Result<cell::Ref<'b,Self::Output>, cell::BorrowError>;
}

pub trait GreedyRead
{
    type Output;

    fn await_greedy_read(&self) -> sync::RwLockReadGuard<Self::Output>; 
}

impl<T> GreedyLock for rc::Rc<cell::RefCell<T>>
{
    type Output = T;

    fn await_greedy_lock<'a:'b,'b>(&'a self) -> cell::RefMut<'b, Self::Output> 
    {
        let target: cell::RefMut<'b, T>;
        loop {
            match self.try_borrow_mut() {
                Ok(inner) => {
                    target = inner;
                    break
                },
                Err(_) => continue
            }
        }
        target
    }

    fn await_greedy_lock_with_timeout<'a:'b,'b>(&'a self, timeout_limit: time::Duration) -> Result<cell::RefMut<'b,Self::Output>, cell::BorrowMutError>
    {
        let mut unsafe_initialised_mem: Result<cell::RefMut<'b,T>, cell::BorrowMutError> = unsafe { std::mem::zeroed() };
        let timer_start_point: time::Instant = time::Instant::now();
        
        while timer_start_point.elapsed() < timeout_limit 
        {
            match self.try_borrow_mut() {
                Ok(inner) => {
                    unsafe_initialised_mem = Ok(inner);
                    break
                },
                Err(e) => {
                    unsafe_initialised_mem = Err(e);
                    continue
                }
            }
        }
        unsafe_initialised_mem
    }
}

impl<'a, T> GreedyLock for &'a rc::Rc<cell::RefCell<T>>
{
    type Output = T;

    fn await_greedy_lock<'b:'c,'c>(&'b self) -> cell::RefMut<'c, Self::Output> 
    {
        let target: cell::RefMut<'c, T>;
        loop {
            match self.try_borrow_mut() {
                Ok(inner) => {
                    target = inner;
                    break
                },
                Err(_) => continue
            }
        }
        target
    }

    fn await_greedy_lock_with_timeout<'b:'c,'c>(&'b self, timeout_limit: time::Duration) -> Result<cell::RefMut<'c,Self::Output>, cell::BorrowMutError>
    {
        let mut unsafe_initialised_mem: Result<cell::RefMut<'c,T>, cell::BorrowMutError> = unsafe { std::mem::zeroed() };
        let timer_start_point: time::Instant = time::Instant::now();
        
        while timer_start_point.elapsed() < timeout_limit 
        {
            match self.try_borrow_mut() {
                Ok(inner) => {
                    unsafe_initialised_mem = Ok(inner);
                    break
                },
                Err(e) => {
                    unsafe_initialised_mem = Err(e);
                    continue
                }
            }
        }
        unsafe_initialised_mem
    }
}

impl<T> GreedyBorrow for rc::Rc<cell::RefCell<T>>
{
    type Output = T;

    fn await_greedy_borrow<'a:'b,'b>(&'a self) -> cell::Ref<'b, Self::Output>
    {
        let target: cell::Ref<'b, T>;
        loop {
            match self.try_borrow() {
                Ok(inner) => {
                    target = inner;
                    break
                },
                Err(_) => continue
            }
        }
        target
    }

    fn await_greedy_borrow_with_timeout<'a:'b,'b>(&'a self, timeout_limit: time::Duration) -> Result<cell::Ref<'b,Self::Output>, cell::BorrowError>
    {
        let mut unsafe_initialised_mem: Result<cell::Ref<'b,T>, cell::BorrowError> = unsafe { std::mem::zeroed() };
        let timer_start_point: time::Instant = time::Instant::now();
        
        while timer_start_point.elapsed() < timeout_limit 
        {
            match self.try_borrow() {
                Ok(inner) => {
                    unsafe_initialised_mem = Ok(inner);
                    break
                },
                Err(e) => {
                    unsafe_initialised_mem = Err(e);
                    continue
                }
            }
        }
        unsafe_initialised_mem
    }
} 

impl<T> GreedyRead for sync::RwLock<T>
{
    type Output = T;

    fn await_greedy_read(&self) -> sync::RwLockReadGuard<T>
    {
        let target: sync::RwLockReadGuard<T>;
        loop {
            match self.try_read() {
                Ok(inner) => {
                    target = inner;
                    break
                },
                Err(_) => continue
            }
        }
        target
    }
}

impl<'a, T> GreedyRead for &'a sync::RwLock<T> 
{
    type Output = T;

    fn await_greedy_read(&self) -> sync::RwLockReadGuard<T>
    {
        let target: sync::RwLockReadGuard<T>;
        loop {
            match self.try_read() {
                Ok(inner) => {
                    target = inner;
                    break
                },
                Err(_) => continue
            }
        }
        target
    }
}