#![feature(try_trait)]
#![feature(test)]
#![feature(lint_reasons)]
#![warn(clippy::all)]

mod environment;
mod message;
mod utils;
mod context;
mod agent;
#[cfg(test)] mod test;

pub use agent::Agent;


/*
 * TODO: 
 * - a registration system to place enlivened agents in the required oracles e.g. AddressCollection
 * - rename Module...
 * - clean utils::sync with macros
 * - coerce insecure hash algorithm for speed
 */