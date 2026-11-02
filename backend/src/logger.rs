use chrono::Local;
use colored::*;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub fn log(service: &str, level: Option<LogLevel>, message: &str) {
    let current_time = Local::now();
    let level_status = match level.unwrap() {
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
    };
    let log = format!(
        "[{0}] [{1}/{2}]: {3}",
        current_time.format("%H:%M:%S"),
        service,
        level_status,
        message,
    );
    let colored_log = match level.unwrap() {
        LogLevel::Info => log.white(),
        LogLevel::Warn => log.yellow(),
        LogLevel::Error => log.red(),
    };

    println!("{}", colored_log)
}

#[macro_export]
macro_rules! info {
    ($service:expr, $msg:expr) => {
        $crate::logger::log($service, Some($crate::logger::LogLevel::Info), $msg);
    };
}

#[macro_export]
macro_rules! warn {
    ($service:expr, $msg:expr) => {
        $crate::logger::log($service, Some($crate::logger::LogLevel::Warn), $msg);
    };
}

#[macro_export]
macro_rules! error {
    ($service:expr, $msg:expr) => {
        $crate::logger::log($service, Some($crate::logger::LogLevel::Error), $msg);
    };
}
