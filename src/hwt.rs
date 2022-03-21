
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
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
pub struct HwTask {
    name: String,
    period: usize,
}

impl HwTask
{
    fn new(n: &str, p: usize) -> Self {
        HwTask{
            name: String::from(n),
            period: p,
        }
    }

    async fn run(self: &mut Self) -> ()
    {
        let mut done = false;
        while !done {
            println!("I'm a {} cycles period hw task", self.period);
            HwFuture::new(&self.name, WaitKind::Time(self.period)).await;
        }
        panic!("This stmt shouldn't be reach");
    }
}

#[derive(Debug)]
pub struct TickKeeper {
    /// Current simulated cycle
    tick: AtomicUsize,
    end_tick: usize,

    /// Number of tick per second
    timescale: usize,

    // HashMap to store the HwTask
    tasks: HashMap<String, HwTask>,
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
