use chrono::Local;
use colored::*;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn label(self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

pub fn log(service: &str, level: LogLevel, message: &str) {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("[{time}] [{service}/{}]: {message}", level.label());

    let colored = match level {
        LogLevel::Info => line.white(),
        LogLevel::Warn => line.yellow(),
        LogLevel::Error => line.red(),
    };

    match level {
        LogLevel::Error => eprintln!("{colored}"),
        _ => println!("{colored}"),
    }
}

#[macro_export]
macro_rules! info {
    ($service:expr, $($arg:tt)*) => {
        $crate::logger::log(
            $service,
            $crate::logger::LogLevel::Info,
            &format!($($arg)*),
        );
    };
}

#[macro_export]
macro_rules! warn {
    ($service:expr, $($arg:tt)*) => {
        $crate::logger::log(
            $service,
            $crate::logger::LogLevel::Warn,
            &format!($($arg)*),
        );
    };
}

#[macro_export]
macro_rules! error {
    ($service:expr, $($arg:tt)*) => {
        $crate::logger::log(
            $service,
            $crate::logger::LogLevel::Error,
            &format!($($arg)*),
        );
    };
}
