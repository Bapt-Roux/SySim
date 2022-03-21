//! HwT
//!
//! Implement a async task that reprsent HW. This task required a tick keeper to be waken up

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct HwTask {
    /// General informations
    name: String,

    // Communication channels
    // TODO

    // Reference to TickKeeper
    // tick_keeper: &TickKeeper,
}

impl HwTask {
}
