use std::sync::{mpsc, Mutex};
use std::sync::mpsc::{Sender};
use std::thread;
use chrono::Utc;
use core_affinity::CoreId;
use log::{LevelFilter, Log, Record, SetLoggerError};

pub struct Logger {
    cpu: usize,
    file: String,
    filter_level: LevelFilter,
    sender: Option<Mutex<Sender<String>>>,
}

impl Logger {
    pub fn new() -> Logger {

        Logger {
            cpu: 1,
            file: "".to_string(),
            filter_level: LevelFilter::Off,
            sender: None,
        }
    }

    pub fn plog(self) {
        self.flush();
    }

    pub fn level(mut self, filter: LevelFilter) -> Logger {
        self.filter_level = filter;
        self
    }

    pub fn cpu(mut self, cpu: usize) -> Logger {
        self.cpu = cpu;
        self
    }

    pub fn init(mut self) -> Result<(), SetLoggerError> {
        let (tx, rx) = mpsc::channel();
        self.sender = Some(Mutex::new(tx));
        let core = self.cpu.clone();
        // receiver: Mutex::new(rx)

        log::set_max_level(self.filter_level);
        log::set_boxed_logger(Box::new(self))?;

        thread::spawn(move || {
            core_affinity::set_for_current(CoreId {id: core});
            loop {
                let msg = rx.recv().unwrap();
                println!("{}",msg);
            }
        });

        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        return self.filter_level != LevelFilter::Off
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("Willow: {} [{}] {}", Utc::now(), record.level() ,record.args());
            match &self.sender {
                Some(tx) => {
                    tx.lock().unwrap().send(msg).unwrap();
                }
                None => ()
            }
        }
    }

    fn flush(&self) {
        println!("Flushing")
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
