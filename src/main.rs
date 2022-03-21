
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
    let tk = hwt::TickKeeper::new(opt.tick, opt.timescale);
    hwt::register(tk);
    // use `TickKeeper::global()` from now on

    // Register two tasks
    let p10 = hwt::HwTask("p10", 10);
    TickKeeper::global().register_hwt("p10", p10);
    let p20 = hwt::HwTask("p20", 20);
    TickKeeper::global().register_hwt("p20", p20);

    // Start simulation loop for 400 cycles
    TickKeeper::global().simulate(400);
}
