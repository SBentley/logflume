# Low latency logging library for Rust 🪵
### Example
```rust
use logflume::{info, Level};
use std::fs;

fn main() {
    logflume::Logger::new()
        .level(Level::Debug)
        .cpu(2)
        .file("my-log-file.log")
        .init()
        .expect("Unable to construct logger");

    for i in 1..1_000_001 {
        info!("number {}", i);
    }
    logflume::logger().flush();
}
```

logflume is an asynchronous logger, it hands of all the formatting and writing of logs to another thread to minimize latency on the calling thread. A blocking call to `logflume::logger::flush()` is needed if you want to wait for all log messages to be processed, it is advisable to do this before the program shuts down to guarantee that all logs are persisted.
