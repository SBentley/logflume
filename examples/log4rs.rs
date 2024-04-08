use std::fs;
use std::path::Path;

use log::info;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main() {
    if Path::new("log4rs.log").exists() {
        fs::remove_file("log4rs.log").expect("Cannot delete test log file.");
    }

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
        .build("log4rs.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    for i in 1..1_000_001 {
        info!("test {}", i);
    }
}
