//! A collection of utilities for the model.

pub mod sync;

pub type Cell<T> = std::rc::Rc<std::cell::RefCell<T>>;