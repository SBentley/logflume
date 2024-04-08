use log::LevelFilter;
use logflume::info;
use std::fs;
use std::path::Path;

fn main() {
    if Path::new("test.log").exists() {
        fs::remove_file("test.log").expect("Cannot delete test log file.");
    }
    logflume::Logger::new()
        .level(LevelFilter::Debug)
        .cpu(2)
        .file("test.log")
        .init()
        .expect("Unable to construct logger");

    for i in 1..1_000_001 {
        info!("test {}", i);
    }
    logflume::logger().flush();
}
