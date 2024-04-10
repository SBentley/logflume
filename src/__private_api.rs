pub use crate::LoggingFunc;
use crate::Level;
pub use std::format_args;

pub fn log(level: Level, func: crate::LoggingFunc) {
    crate::logger().log(level, func)
}
