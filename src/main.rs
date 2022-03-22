
use std::sync::mpsc::channel;
use structopt::StructOpt;

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

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("User Options:\n {:?}", opt);

    // Create and register the global TickKeeper
    hwt::TickKeeper::new(opt.tick, opt.timescale).register();

    // Create the hwScheduler
    let (tx, rx) = channel();
    let mut scheduler = hwt::HwScheduler::new(rx);

    // Create HwTasks with random period, register them in the scheduler and 
    // spawned them on tokio runtime
    // let tasks: Vec<hwt::HwTask> = Vec::new();
    let mut tasks = Vec::new();
    for t in 0..opt.coworker {
        let p = 20*(t+1) as usize;
        let tx = tx.clone();
        tasks.push(hwt::HwTask::new(format!("HwTask_P{}",p), hwt::WaitKind::Time(p), tx));
    }

    // Start simulation loop for 400 cycles
    scheduler.simulate(400);
}
