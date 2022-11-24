extern crate core;

use chrono::Utc;
use core_affinity::CoreId;
use crossbeam_channel;
use crossbeam_channel::internal::SelectHandle;
use log::{LevelFilter, Log, Record, SetLoggerError};
use std::fs::File;
use std::io::Write;
use std::thread;

pub struct Logger {
    cpu: usize,
    file_path: Option<String>,
    filter_level: LevelFilter,
    sender: Option<crossbeam_channel::Sender<LogCommand>>,
}

pub enum LogCommand {
    Msg(String),
    Flush(crossbeam_channel::Sender<()>),
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            cpu: 1,
            file_path: None,
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

    pub fn file(mut self, file: &str) -> Logger {
        self.file_path = Some(file.to_string());
        self
    }

    pub fn init(mut self) -> Result<(), SetLoggerError> {
        let (tx, rx) = crossbeam_channel::unbounded();

        self.sender = Some(tx);
        let core = self.cpu.clone();
        let file_path = self.file_path.take();
        let mut file = match file_path {
            Some(f) => Some(File::create(f).unwrap()),
            None => None,
        };

        let _a = thread::spawn(move || {
            core_affinity::set_for_current(CoreId { id: core });
            loop {
                let mut last_msg = String::new();
                match rx.try_recv() {
                    Ok(cmd) => {
                        if let Some(ref mut f) = file {
                            match cmd {
                                LogCommand::Msg(msg) => {
                                    last_msg = msg.clone();
                                    f.write(msg.as_bytes()).unwrap();
                                }
                                LogCommand::Flush(tx) => {
                                    println!("last_msg: {}", last_msg);
                                    f.flush().unwrap();
                                    tx.send(());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        match e {
                            crossbeam_channel::TryRecvError::Empty => {
                                if let Some(ref mut f) = file {
                                    f.flush().unwrap();
                                }
                            },
                            crossbeam_channel::TryRecvError::Disconnected => todo!(),
                        }
                        println!("recv err");
                        if let Some(ref mut f) = file {
                            f.write("recv err".as_bytes()).unwrap();
                        }
                    }
                }
            }
        });
        log::set_max_level(self.filter_level);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        return self.filter_level != LevelFilter::Off;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!(
                "Willow: {} [{}] {}\n",
                Utc::now(),
                record.level(),
                record.args()
            );
            match &self.sender {
                Some(tx) => {
                    tx.send(LogCommand::Msg(msg)).unwrap();
                }
                None => (),
            }
        }
    }

    fn flush(&self) {
        println!("Flushing");
        if let Some(tx) = &self.sender { 
            tx.send(LogCommand::Flush(crossbeam_channel::bounded(1))).unwrap();
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.flush();
        println!("drop")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
