extern crate core;

use chrono::{DateTime, Utc};
use core_affinity::CoreId;
use log::LevelFilter;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread;

pub mod __private_api;
pub mod macros;
pub use log::Level;

static mut LOGGER: Option<&Logger> = None;

struct LogMetaData {
    level: Level,
    time: DateTime<Utc>,
    func: LoggingFunc,
}

#[derive(Debug)]
pub enum LoggerError {
    InitialisationError,
}

impl std::error::Error for LoggerError {}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoggerError::InitialisationError => write!(f, "Error initialising algo logger"),
        }
    }
}

enum LogCommand {
    Msg(LogMetaData),
    Flush(crossbeam_channel::Sender<()>),
}

pub struct LoggingFunc {
    data: Box<dyn Fn() -> String + Send + 'static>,
}

impl LoggingFunc {
    #[allow(dead_code)]
    pub fn new<T>(data: T) -> LoggingFunc
    where
        T: Fn() -> String + Send + 'static,
    {
        return LoggingFunc {
            data: Box::new(data),
        };
    }
    fn invoke(self) -> String {
        (self.data)()
    }
}

impl fmt::Debug for LoggingFunc {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct Logger {
    cpu: usize,
    buffer_size: usize,
    file_path: Option<String>,
    filter_level: LevelFilter,
    sender: Option<crossbeam_channel::Sender<LogCommand>>,
}

impl Logger {
    pub fn new() -> Logger {
        let cpus = core_affinity::get_core_ids();
        let cpu = match cpus {
            Some(c) => c.last().unwrap().id,
            None => 0,
        };
        Logger {
            cpu,
            buffer_size: 0,
            file_path: None,
            filter_level: LevelFilter::Off,
            sender: None,
        }
    }

    pub fn level(mut self, filter: LevelFilter) -> Logger {
        self.filter_level = filter;
        self
    }

    pub fn cpu(mut self, cpu: usize) -> Logger {
        self.cpu = cpu;
        self
    }

    pub fn buffer_size(mut self, buf_size: usize) -> Logger {
        self.buffer_size = buf_size;
        self
    }

    pub fn file(mut self, file: &str) -> Logger {
        self.file_path = Some(file.to_string());
        self
    }

    pub fn init(mut self) -> Result<(), LoggerError> {
        let (tx, rx) = match self.buffer_size {
            0 => crossbeam_channel::unbounded(),
            _ => crossbeam_channel::bounded(self.buffer_size),
        };

        self.sender = Some(tx);
        let file_path = self.file_path.clone();
        let file =
            File::create(file_path.unwrap()).map_err(|_| LoggerError::InitialisationError)?;
        let mut buffered_writer = BufWriter::new(file);
        let core = self.cpu;

        let _a = thread::spawn(move || {
            core_affinity::set_for_current(CoreId { id: core });
            loop {
                match rx.try_recv() {
                    Ok(cmd) => {
                        Self::process_log_command(&mut buffered_writer, cmd);
                    }
                    Err(e) => match e {
                        crossbeam_channel::TryRecvError::Empty => {
                            let _ = buffered_writer.flush();
                        }
                        crossbeam_channel::TryRecvError::Disconnected => {
                            let _ = buffered_writer
                                .write_all("Logging channel disconnected".as_bytes());
                        }
                    },
                }
            }
        });

        log::set_max_level(self.filter_level);
        unsafe {
            let boxed_logger = Box::new(self);
            LOGGER = Some(Box::leak(boxed_logger));
        }
        Ok(())
    }

    fn process_log_command(buffered_file_writer: &mut BufWriter<File>, cmd: LogCommand) {
        match cmd {
            LogCommand::Msg(msg) => {
                let log_msg = format!("{} [{}] {}\n", msg.time, msg.level, msg.func.invoke());
                let _ = buffered_file_writer.write_all(log_msg.as_bytes());
            }
            LogCommand::Flush(tx) => {
                let _ = buffered_file_writer.flush();
                let _ = tx.send(());
            }
        }
    }

    pub fn log(&self, level: Level, func: LoggingFunc) {
        match &self.sender {
            Some(tx) => {
                tx.send(LogCommand::Msg(LogMetaData {
                    level,
                    time: Utc::now(),
                    func,
                }))
                .unwrap();
            }
            None => (),
        }
    }

    /// Blocking
    pub fn flush(&self) {
        if let Some(tx) = &self.sender {
            let (flush_tx, flush_rx) = crossbeam_channel::bounded(1);
            tx.send(LogCommand::Flush(flush_tx)).ok();
            let _ = flush_rx.recv();
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.flush();
    }
}

pub fn logger() -> &'static Logger {
    unsafe { LOGGER.unwrap() }
}

#[cfg(test)]
mod tests {
    use crate::Logger;
    use crate::{debug, error, info, warn};

    #[test]
    pub fn test_log() {
        Logger::new().file("test.log").init().unwrap();
        info!("hello {} {}", "world", 123);
        warn!("hello world");
        debug!("debug log");
        error!("Something went wrong!");
    }
}
