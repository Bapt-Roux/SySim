
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use once_cell::sync::OnceCell;
use crossbeam::atomic::AtomicCell;
use debugless_unwrap::DebuglessUnwrap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver,Sender};

// WaitKind -------------------------------------------------------------------
/// WaitKind define the wait type use for the current job
#[derive(Debug)]
pub enum WaitKind {
    Time(usize),
    Event((String, usize)),
}

// HwJob ----------------------------------------------------------------------
/// HwJob: Structure use to pass wakeup information to the HwScheduler
#[derive(Debug)]
pub struct HwJob {
    wait_evt: WaitKind,
    waker: Waker,
}

// TickKeeper ----------------------------------------------------------------
/// TickKeeper: Data sharing point between Scheduler and Hw tasks
/// Enable every HwTask to access global simulation time and update the number of infligth tasks
/// Warn: This struct is instanciated once and made global with once_cell
pub struct TickKeeper {
    pub tick: AtomicUsize,
    timescale: usize,
    pub inflight_hwt: AtomicUsize,
    scheduler_waker: AtomicCell<Option<Waker>>,
}
pub static TICK_KEEPER: OnceCell<TickKeeper> = OnceCell::new();

impl TickKeeper {
    /// Constructs a new `HwScheduler`.
    ///
    /// `cur_tick` Start from the given tick
    /// `timescale` Used time resolution
    ///
    pub fn new(cur_tick: usize, timescale: usize) -> Self {
        println!("Create a new TickKeeper @{}[{}]", cur_tick, timescale);
        TickKeeper {
            tick: AtomicUsize::new(cur_tick),
            timescale: timescale,
            inflight_hwt: AtomicUsize::new(0),
            scheduler_waker: AtomicCell::new(None),
        }
    }

    pub fn register(self: Self) -> () {
        TICK_KEEPER.set(self).debugless_unwrap();
    }

    pub fn global() -> &'static TickKeeper {
        TICK_KEEPER.get().expect("HwScheduler is not initialized")
    }

    pub fn register_waker(waker: Waker) -> () {
        let waker_cell = &TICK_KEEPER.get().unwrap().scheduler_waker;
        waker_cell.store(Some(waker));
    }
}

/// Global scope function to retrieved some simulation tick
/// A simple function helper
pub fn cur_tick() -> usize {
    let tk = TickKeeper::global();
    tk.tick.load(Ordering::SeqCst)
}

// HwScheduler ----------------------------------------------------------------
/// HwScheduler: Implement a async task that handle the simulated time update 
/// and the inter-task events.
/// Communication from HwTasks are retrieved through mpsc channel
#[derive(Debug)]
pub struct HwScheduler {
    hw_rx: Receiver<HwJob>,
    pending_hwt: Vec<HwJob>,
}

impl HwScheduler {
    /// Constructs a new `HwScheduler`.
    ///
    /// `cur_tick` Start from the given tick
    /// `timescale` Used time resolution
    ///
    pub fn new(rx: Receiver<HwJob>) -> Self {
        println!("Create a new HwScheduler");
        HwScheduler {
            hw_rx: rx,
            pending_hwt: Vec::new(),
        }
    }

    fn get_next_tick(self: &Self) -> usize {
        match &self.pending_hwt[0].wait_evt {
            WaitKind::Time(t) => { *t },
            WaitKind::Event((n,t)) => { *t },
        }
    }

    pub async fn simulate(self: &mut Self, duration: usize) -> () {
        println!("{}: Start simulation loop for {} tick", cur_tick(), duration);

        let mut hw_task = TickKeeper::global().inflight_hwt.load(Ordering::SeqCst);
        loop {
            // Wait for inflight hw task to complete
            HwSchedFuture{}.await;

            println!("{}: Get messages from {} tasks", cur_tick(), hw_task);
            // Retrieved jobs from previous inflight task
            for i in 0..hw_task {
                let job = self.hw_rx.recv().unwrap();
                self.pending_hwt.push(job);
            }
            // Ordered job vector
            self.pending_hwt.sort_by(|a, b| {
                let tick_a = match &a.wait_evt {
                    WaitKind::Time(t) => { t },
                    WaitKind::Event((n,t)) => { t },
                };

                let tick_b = match &b.wait_evt {
                    WaitKind::Time(t) => { t },
                    WaitKind::Event((n,t)) => { t },
                };

                tick_a.cmp(&tick_b)
            });

            if 0 == self.pending_hwt.len() {
                panic!("{}: Hw task vector is empty nothing to simulate.", cur_tick());
            }
            let next_tick = self.get_next_tick();
            if next_tick > duration {
                break;
            }

            // Loop over event that are below the next_tick
            while next_tick < self.get_next_tick() {                // Check loop condition
                // Pop the job and process accordingly
                let job = self.pending_hwt.first().unwrap();
                match job {
                    HwJob {wait_evt: WaitKind::Time(t), waker: w} => {
                        // increase the inflight number and local counter
                        TickKeeper::global().inflight_hwt.fetch_add(1, Ordering::SeqCst);
                        hw_task+=1;
                        // wake the task
                        w.wake_by_ref();
                    },
                    HwJob {wait_evt: WaitKind::Event((n,t)), waker: w} => {
                        todo!();
                    }
                }
            }

            // Update the tick and wait for the next simulation period
            TickKeeper::global().tick.store(next_tick, Ordering::SeqCst);
        };
    }

    /// Generate the given event id at tick
    pub fn notify(self: &mut Self, name: String, tick: usize) -> () {
        println!(" Event id:{} fired, notify associated HwTasks", name);
        todo!();
    }
}

// HwSchedFuture
/// Future with particular handle to wake up scheduler from HwTask
pub struct HwSchedFuture;

impl Future for HwSchedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        println!("{}: HwScheduler polled", cur_tick());
        if 0 == TickKeeper::global().inflight_hwt.load(Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            // Register waker in the TickKeeper for future access by HwTask
            TickKeeper::register_waker(cx.waker().clone());
            Poll::Pending
        }
    }
}

// HwTask ---------------------------------------------------------------------
/// HwTask: Implement a async task that represent Hw component execution loop
/// In practice this should be a trait implemented by multiple component structures
///
#[derive(Debug)]
pub struct HwTask {
    name: String,
    kind: WaitKind,
    hw_tx: Sender<HwJob>,
}

impl HwTask {
    pub fn new(name: String, kind: WaitKind, tx: Sender<HwJob>) -> Self {
        // Considered the task as inflight at the beginning
        let br = TickKeeper::global().inflight_hwt.load(Ordering::SeqCst);
        let inflight = TickKeeper::global().inflight_hwt.fetch_add(1, Ordering::SeqCst);
        let rb = TickKeeper::global().inflight_hwt.load(Ordering::SeqCst);

        println!("{}: Debug inflight counter@{:p} {}::{}::{}", cur_tick(), &TickKeeper::global().inflight_hwt, br, inflight, rb);
        println!("{}: Create HwTask {}[{:?}] => inflight {}", cur_tick(), name, kind, inflight);

        HwTask {
            name: name,
            kind: kind,
            hw_tx: tx,
        }
    }

    pub async fn run(self: &mut Self) -> () {
        loop {
            HwFuture::new(self).await;
            println!("{}: Execute HwTask {}[{:?}]", cur_tick(), self.name, self.kind);
        }
    }
}

// HwFuture ------------------------------------------------------------------
/// Implement a future that registered itself in the HwScheduler for waking up
/// bypass the standard tokio wakeup mechanisms for Hw simulation purpose
///
pub struct HwFuture<'p> {
    parent: &'p HwTask,
}

impl<'p> HwFuture<'p> {
    pub fn new(parent: &'p mut HwTask) -> Self {
        HwFuture {
            parent: parent,
        }
    }

    async fn wait_for(tick: usize) -> () {
        todo!()
    }

    async fn wait_event(name: &str) -> () {
        todo!()
    }
}

impl<'p> Future for HwFuture<'p> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        println!("{}: HwFuture polled", cur_tick());

        let ltick = cur_tick();
        match &self.parent.kind {
            WaitKind::Time(t) => {
                // Ready path
                if ltick >= *t {
                    Poll::Ready(())
                } else {
                    let job = HwJob {
                        wait_evt: WaitKind::Time(*t),
                        waker: cx.waker().clone(),
                    };
                    // Send the job and update inflight cnt
                    self.parent.hw_tx.send(job).unwrap();
                    let remaining_hwt = TickKeeper::global().inflight_hwt.fetch_sub(1, Ordering::SeqCst);
                    // println!("",);
                    if 0 == remaining_hwt {
                        TickKeeper::global().scheduler_waker.take().unwrap().wake_by_ref();
                    }
                    Poll::Pending
                }
            },
            WaitKind::Event((n,t)) => {
                todo!();
            },
        }
    }
}
