//! TickKeeper
//!
//! Implement a async task that wait to all inflight hwt, update the current tick and wake up the
//! ready hwt

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::sync_channel;

struct TickKeeper {
    /// Simulated time representation
    tick: AtomicUsize, /// Current simulated cycle
    timescale: u64, /// Number of tick per second
}

impl TickKeeper {
    /// Constructs a new `TickKeeper`.
    ///
    /// `cur_tick` Start from the given tick
    /// `timescale` Used time resolution
    ///
    fn new(cur_tick: u64, timescale: u64) -> Self {
        println!("Create a new TimeKeeper @{}[{}]", cur_tick, timescale);
        TickKeeper {
            tick = AtomicUsize::new(cur_tick),
            timescale = timescale,
        }
    }

}
