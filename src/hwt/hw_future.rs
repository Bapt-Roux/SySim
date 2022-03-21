//! Hw Future
//!
//! Implement a biased future that rely on a TickKeeper to be woken up in time

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};


pub enum WaitKind {
    Time(u64),
    Event(String),
}

pub struct HwFuture<'p> {
    /// Reference to parent name for debug
    name: &'p str,

    /// Expected wakeup kind
    kind: WaitKind,
}


impl<'p> Future for HwFuture<'p> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        println!("HwFuture polled");

        match self.kind {
            WaitKind::Time(t) => {
                // Ready path
                Poll::Ready(())

                // Pending path
                // cx.waker().wake_by_ref();
                // Poll::Pending
            },
            // WaitKind::Event(n) => {
            // },
            _ => { panic!("Unknown kind"); }
        }
    }
}

impl<'p> HwFuture<'p> {
    pub fn new(p: &'p str, k: WaitKind) -> Self {
        HwFuture {
            name: p,
            kind: k,
        }
    }

    pub async fn wait_for_tick(tick: u64) -> () {
        todo!()
    }


    pub async fn wait_for_event(name: &str) -> () {
        todo!()
    }
}

