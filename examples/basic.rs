use chrono::{Utc};
use log::{info, LevelFilter};
use std::fs;
use std::path::Path;

fn main() {
    if Path::new("test.log").exists() {
        fs::remove_file("test.log").expect("Cannot delete test log file.");
    }
    willow::Logger::new()
        .level(LevelFilter::Debug)
        .cpu(7)
        .file("test.log")
        .init()
        .expect("Unable to construct logger");

    info!("{}", Utc::now());    
    for i in 1..1_000_000 {
        info!("test {}", i, );
    }
    log::logger().flush();


}
