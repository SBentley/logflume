use log::LevelFilter;
use std::fs;
use std::path::Path;
use willow::info;

fn main() {
    if Path::new("test.log").exists() {
        fs::remove_file("test.log").expect("Cannot delete test log file.");
    }
    willow::Logger::new()
        .level(LevelFilter::Debug)
        .cpu(2)
        .file("test.log")
        .init()
        .expect("Unable to construct logger");

    for i in 1..1_000_001 {
        info!("test {}", i);
    }
    willow::logger().flush();
}
