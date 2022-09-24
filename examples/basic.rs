use log::{info, LevelFilter};

fn main() {
    let logger = willow::Logger::new().level(LevelFilter::Debug).init();
    info!("test");

}