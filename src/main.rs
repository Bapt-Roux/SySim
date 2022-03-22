
use std::sync::mpsc::channel;
use structopt::StructOpt;
use rand::Rng;

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

    // Create and register the global HwScheduler
    let (rx, tx) = channel();
    hwt::HwScheduler::new(opt.tick, opt.timescale, rx).register();

    // Create HwTasks with random period, register them in the scheduler and 
    // spawned them on tokio runtime
    let mut rng = rand::thread_rng();
    let tasks: Vec<hwt::HwTask>;
    for t in 0..opt.coworker {
        let p = rng.gen_range(1..200);
        let tx = tx.clone();
        tasks.push(hwt::HwTask::new(format!("HwTask_P{}", hwt::WaitKind::Tick(p)), tx));
    }

    // Start simulation loop for 400 cycles
    HwScheduler::global().simulate(400);
}
