//! A concurrent unbounded blocking queue where each element can only be removed when
//! its delay expires.

#![warn(missing_docs)]

extern crate parking_lot;

mod delayed;
mod delay_queue;

pub use delayed::{Delay, Delayed};
pub use delay_queue::DelayQueue;
