use std::thread::{sleep, Thread};
use std::time::Duration;
use log::{info, LevelFilter};

fn main() {
    willow::Logger::new()
        .level(LevelFilter::Debug)
        .cpu(7)
        .file("target/test.log")
        .init().expect("Unable to construct logger");
    let core_ids = core_affinity::get_core_ids().unwrap();
    println!("{:?}", core_ids);
    info!("test");
    log::logger().flush();

}
