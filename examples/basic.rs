use logflume::{info, Level};
use std::fs;
use std::path::Path;

fn main() {
    if Path::new("test.log").exists() {
        fs::remove_file("test.log").expect("Cannot delete test log file.");
    }
    logflume::Logger::new()
        .level(Level::Debug)
        .cpu(2)
        .file("test.log")
        .sleep_duration_millis(100)
        .thread_name("Example - logging thread")
        .init()
        // .buffer_size(2_000_000)
        .expect("Unable to construct logger");

    for i in 1..1_000_001 {
        info!("test {}", i);
    }
    // logflume::logger().flush();
}
