use std::thread::{sleep, Thread};
use std::time::Duration;
use log::{info, LevelFilter};

fn main() {
    willow::Logger::new()
        .level(LevelFilter::Debug)
        .cpu(7)
        .file("test.log")
        .init().expect("Unable to construct logger");
    let core_ids = core_affinity::get_core_ids().unwrap();
    println!("{:?}", core_ids);
    info!("test");

    sleep(Duration::from_secs(5));
    println!("shutting down");

}