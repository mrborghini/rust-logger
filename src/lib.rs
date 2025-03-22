mod colors;
mod log_type;
pub mod severity;
mod log;

use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{Local, TimeZone, Utc};
use colors::Colors;
use log::Log;
use log_type::LogType;
pub use severity::Severity;

pub struct Logger {
    type_name: String,
}

impl Logger {
    pub fn new<S: AsRef<str>>(type_name: S) -> Self {
        let type_name = type_name.as_ref().to_string();
        Self { type_name }
    }

    fn log(&self, mut info: Log) {
        info.application_name = env!("CARGO_PKG_NAME").to_string();
        info.app_version = env!("CARGO_PKG_VERSION").to_string();

        info.time = self.get_time();

        let message = format!(
            "[{} - {} - (v{}) ({} - {}) - {}]: {}",
            info.log_type,
            info.severity,
            info.app_version,
            info.application_name,
            info.type_name,
            self.format_time(info.time),
            info.message
        );

        match info.log_type {
            LogType::Info => println!("{}{}{}", Colors::ok_blue(), message, Colors::normal()),
            LogType::Debug => if let Ok(value) = env::var("LOGGER_DEBUG") {
                if value.to_lowercase() == "true" {
                    println!("{}{}{}", Colors::ok_green(), message, Colors::normal());
                }
            },
            LogType::Warning => println!("{}{}{}", Colors::warning(), message, Colors::normal()),
            LogType::Error => println!("{}{}{}", Colors::error(), message, Colors::normal()),
        }
    }

    pub fn info<S: AsRef<str>>(&self, message: S) {
        let message = message.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            Severity::None,
            LogType::Info,
            0,
            message,
            self.type_name.clone(),
            "".to_string(),
        );
        self.log(info);
    }

    pub fn debug<S: AsRef<str>>(&self, message: S) {
        let message = message.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            Severity::None,
            LogType::Debug,
            0,
            message,
            self.type_name.clone(),
            "".to_string(),
        );
        self.log(info);
    }

    pub fn warning<S: AsRef<str>>(&self, message: S, severity: Severity) {
        let message = message.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            severity,
            LogType::Warning,
            0,
            message,
            self.type_name.clone(),
            "".to_string(),
        );
        self.log(info);
    }

    pub fn error<S: AsRef<str>>(&self, message: S, severity: Severity) {
        let message = message.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            severity,
            LogType::Error,
            0,
            message,
            self.type_name.clone(),
            "".to_string(),
        );
        self.log(info);
    }

    fn get_time(&self) -> u64 {
        let start = SystemTime::now();

        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        since_the_epoch.as_secs()
    }

    fn format_time(&self, unix_time: u64) -> String {
        let utc_datetime = Utc
            .timestamp_opt(unix_time as i64, 0)
            .single()
            .expect("Invalid timestamp");

        let local_datetime = utc_datetime.with_timezone(&Local);

        local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
