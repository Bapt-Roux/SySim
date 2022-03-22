
use futures::future::join_all;
use std::sync::mpsc::channel;
use structopt::StructOpt;
use tokio::task;
use debugless_unwrap::DebuglessUnwrap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod hwt;

/// Define CLI arguments
#[derive(Debug, StructOpt)]
#[structopt(about = "Simulating Hw in Rust: PoC based on Rust-Async/ Tokio.")]
struct Opt {
    /// Number of coworker to generate
    #[structopt(short)]
    coworker: u32,

    /// Starting simulation tick
    #[structopt(long)]
    tick: usize,

    /// Number of tick per second (resolution)
    #[structopt(long)]
    timescale: usize,
}

// Global simulation state
// static TICK_KEEPER: OnceCell<TickKeeper> = OnceCell::new();

#[tokio::main(worker_threads = 1)]
async fn main() {
    let opt = Opt::from_args();
    println!("User Options:\n {:?}", opt);

    // Create and register the global TickKeeper
    hwt::TickKeeper::new(opt.tick, opt.timescale).register();

    // Create the hwScheduler
    let (tx, rx) = channel();
    let mut scheduler = hwt::HwScheduler::new(rx);
    // Configure a simulation loop for 400 cycles
    let sched_fut = scheduler.simulate(400);

    // Circumvent Send issue with local task set and spawn local
    let local = task::LocalSet::new();

    // Spawn and Merge hw_task in one future
    let cpn_fut = local.run_until( async move {
        // Create HwTasks register spawned them locally on tokio runtime and store future in vector
        let mut task_fut = Vec::new();
        for t in 0..opt.coworker {
            let p = 20*(t+1) as usize;
            let tx = tx.clone();
            task_fut.push(tokio::task::spawn_local(async move {
                        let mut task = hwt::HwTask::new(format!("HwTask_P{}",p), hwt::WaitKind::Time(p), tx);
                        task.run().await;
                        }));
        }

        // Join all task futures
        join_all(task_fut).await;
    });

    // Wait for the simulation to run until completion
    tokio::join!(cpn_fut, sched_fut);

}
