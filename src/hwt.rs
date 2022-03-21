
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::sync_channel;


#[derive(Debug)]
pub enum WaitKind {
    Time(usize),
    Event(String),
}

#[derive(Debug)]
pub struct HwFuture<'p> {
    /// Reference to parent name for debug
    name: &'p str,

    /// Expected wakeup kind
    kind: WaitKind,
}

impl<'p> HwFuture<'p> {
    pub fn new(p: &'p str, k: WaitKind) -> Self {
        HwFuture {
            name: p,
            kind: k,
        }
    }

    async fn wait_for_tick(tick: usize) -> () {
        todo!()
    }


    async fn wait_for_event(name: &str) -> () {
        todo!()
    }
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
                if cur_tick() >= t {
                    Poll::Ready(())
                } else {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            },
            // WaitKind::Event(n) => {
            // },
            _ => { panic!("Unknown kind"); }
        }
    }
}


#[derive(Debug)]
pub struct TickKeeper {
    /// Current simulated cycle
    tick: AtomicUsize,
    /// Number of tick per second
    timescale: usize,
}
static TICK_KEEPER: OnceCell<TickKeeper> = OnceCell::new();

impl TickKeeper {
    /// Constructs a new `TickKeeper`.
    ///
    /// `cur_tick` Start from the given tick
    /// `timescale` Used time resolution
    ///
    pub fn new(cur_tick: usize, timescale: usize) -> Self {
        println!("Create a new TimeKeeper @{}[{}]", cur_tick, timescale);
        TickKeeper {
            tick: AtomicUsize::new(cur_tick),
            timescale: timescale,
        }
    }

    pub fn global() -> &'static TickKeeper {
        TICK_KEEPER.get().expect("TickKeeper is not initialized")
    }

}

pub fn register(tk: TickKeeper) -> () 
{
    TICK_KEEPER.set(tk).unwrap();
}


pub fn cur_tick() -> usize
{
    let tk = TickKeeper::global();
    tk.tick.load(Ordering::Relaxed)
}
