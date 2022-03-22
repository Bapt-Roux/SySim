
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::mpsc::sync_channel;


// Implement the Tick Keeper struct
// Handle the simulation time and expose a simulate functtion
// Warn: This struct is instanciated once and made global with once_cell
#[derive(Debug)]
pub struct TickKeeper {
    /// Current simulated cycle
    tick: AtomicUsize,

    /// Number of tick per second
    timescale: usize,

    // Communication HwTask and counter for number of inflight task
    nb_hwt: u64,
    pdg_hwt: u64,
    rx: Receiver<(usize, &Waker)>, // todo
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

    pub fn register_hwt(self: &mut Self, n:&str, hwt: HwTask) -> (){
        self.tasks.insert(String::from(n), hwt);
    }

    pub fn simulate(self: &mut Self, end: usize) -> (){
        self.end_tick = end;

        // Rearm associated HwTasks
        for (key, value) in &*self.tasks {
            println!("Start {}", key);
            value.init();
        }
    }

    pub fn update_tick(self: &mut Self) -> () {

    }

    pub fn cur_tick() -> usize {
        let tk = TickKeeper::global();
        tk.tick.load(Ordering::Relaxed)
    }


    pub fn register(tk: TickKeeper) -> () {
        TICK_KEEPER.set(tk).unwrap();
    }
}
