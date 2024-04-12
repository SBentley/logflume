extern crate core;

use chrono::{DateTime, Utc};
use core_affinity::CoreId;
use lockfree::channel::spsc;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread;

pub mod __private_api;
pub mod macros;

static mut LOGGER: Option<&mut Logger> = None;

#[derive(Debug)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warn => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Debug)]
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
            LoggerError::InitialisationError => write!(f, "Error during initialisation"),
        }
    }
}

#[derive(Debug)]
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
    filter_level: Level,
    sender: spsc::Sender<LogCommand>,
}

impl Logger {
    pub fn new() -> Logger {
        let cpus = core_affinity::get_core_ids();
        let cpu = match cpus {
            Some(c) => c.last().unwrap().id,
            None => 0,
        };
        let tx = spsc::create::<LogCommand>().0;
        Logger {
            cpu,
            buffer_size: 0,
            file_path: None,
            filter_level: Level::Info,
            sender: tx,
        }
    }

    pub fn level(mut self, filter: Level) -> Logger {
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
        let (tx, mut rx) = spsc::create::<LogCommand>();

        self.sender = tx;
        let file_path = self.file_path.clone();
        let file =
            File::create(file_path.unwrap()).map_err(|_| LoggerError::InitialisationError)?;
        let mut buffered_writer = BufWriter::new(file);
        let core = self.cpu;

        let _a = thread::spawn(move || {
            core_affinity::set_for_current(CoreId { id: core });
            loop {
                match rx.recv() {
                    Ok(cmd) => {
                        Self::process_log_command(&mut buffered_writer, cmd);
                    }
                    Err(e) => match e {
                        spsc::RecvErr::NoMessage => {}
                        spsc::RecvErr::NoSender => {
                            let _ = buffered_writer
                                .write_all("Logging channel disconnected".as_bytes());
                        }
                    },
                }
            }
        });

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

    pub fn log(&mut self, level: Level, func: LoggingFunc) {
        self.sender
            .send(LogCommand::Msg(LogMetaData {
                level,
                time: Utc::now(),
                func,
            }))
            .unwrap();
    }

    /// Blocking
    pub fn flush(&mut self) {
        let (flush_tx, flush_rx) = crossbeam_channel::bounded(1);
        self.sender.send(LogCommand::Flush(flush_tx)).ok();
        let _ = flush_rx.recv();
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.flush();
    }
}

pub fn logger() -> &'static mut Logger {
    unsafe { LOGGER.as_mut().unwrap() }
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
