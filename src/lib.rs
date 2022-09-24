use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    pub cpu_id: u32
}

impl Logger {

}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        todo!()
    }

    fn log(&self, record: &Record) {

    }

    fn flush(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
