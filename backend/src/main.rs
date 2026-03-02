
mod gpu_ai;
mod cluster;
mod latency;
mod fpga;
mod stat_arb;

#[tokio::main]
async fn main() {
    println!("FED GRID INSTITUTIONAL ULTIMATE STARTED");

    tokio::join!(
        gpu_ai::start(),
        cluster::start_node(),
        latency::optimize(),
        fpga::start(),
        stat_arb::run()
    );
}
