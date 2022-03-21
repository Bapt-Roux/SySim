
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

    let fut = hwt::HwFuture::new("main", hwt::WaitKind::Time(4));
    fut.await;
}
