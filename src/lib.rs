use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    cpu: u16,
    file: String,
    filter_level: LevelFilter,
    // enabled: bool
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            cpu: 1,
            file: "".to_string(),
            filter_level: LevelFilter::Off,
            // enabled: true
        }
    }

    pub fn level(mut self, filter: LevelFilter) -> Logger {
        self.filter_level = filter;
        self
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.filter_level);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }

}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        return self.filter_level != LevelFilter::Off
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Willow: {}", record.args())
        }
    }

    fn flush(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
