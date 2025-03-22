use super::{LogType, Severity};

pub struct Log {
    pub application_name: String,
    pub severity: Severity,
    pub log_type: LogType,
    pub time: u64,
    pub message: String,
    pub type_name: String,
    pub app_version: String,
}

impl Log {
    pub fn new(
        application_name: String,
        severity: Severity,
        log_type: LogType,
        time: u64,
        message: String,
        type_name: String,
        version: String,
    ) -> Self {
        Self {
            application_name,
            severity,
            log_type,
            time,
            message,
            type_name,
            app_version: version,
        }
    }
}
