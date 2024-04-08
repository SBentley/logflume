#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt:expr, $($arg:tt)+) => {
        let func = $crate::__private_api::LoggingFunc::new(move || {
            format!($fmt, $($arg)*)
        });
        $crate::__private_api::log($lvl, func);
    };

    ($lvl:expr, $fmt:expr) => {
        let func = $crate::__private_api::LoggingFunc::new(move || {
            format!($fmt)
        });
        $crate::__private_api::log($lvl, func);
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Error, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Error, expr))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Warn, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Warn, expr))

}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Info, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Info, expr))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Debug, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Debug, expr))
}
