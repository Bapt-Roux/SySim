
use structopt::StructOpt;
use crate::hwt::hw_future::*;

pub mod hwt;
/// Define CLI arguments
#[derive(Debug, StructOpt)]
#[structopt(about = "Simulating Hw in Rust: PoC based on Rust-Async/ Tokio.")]
struct Opt {
    /// Number of coworker to generate
    #[structopt(short)]
    coworker: u32,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("User Options:\n {:?}", opt);

    let fut = HwFuture::new("main", WaitKind::Time(4));
    fut.await;
}
