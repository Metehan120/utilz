#[cfg(not(feature = "async"))]
use std::sync::RwLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(feature = "async")]
use async_trait::async_trait;
#[cfg(feature = "async")]
use once_cell::sync::Lazy;
#[cfg(feature = "async")]
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Debug,
    Info,
    Warn,
}

struct Logger {
    message: String,
    level: LogLevel,
    time: SystemTime,
}

pub struct Log;

#[cfg(feature = "async")]
static LOGS: Lazy<RwLock<Vec<Logger>>> = Lazy::new(|| RwLock::new(Vec::new()));
#[cfg(feature = "async")]
static LOG_LEVEL: Lazy<RwLock<LogLevel>> = Lazy::new(|| RwLock::new(LogLevel::Info));

#[cfg(not(feature = "async"))]
static LOGS: RwLock<Vec<Logger>> = RwLock::new(Vec::new());
#[cfg(not(feature = "async"))]
static LOG_LEVEL: RwLock<LogLevel> = RwLock::new(LogLevel::Debug);

#[cfg(not(feature = "async"))]
fn get_level() -> LogLevel {
    *LOG_LEVEL.read().unwrap()
}

#[cfg(feature = "async")]
async fn get_level() -> LogLevel {
    *LOG_LEVEL.read().await
}

#[cfg(not(feature = "async"))]
fn level_priority(level: LogLevel) -> u8 {
    match level {
        LogLevel::Error => 1,
        LogLevel::Warn => 2,
        LogLevel::Info => 3,
        LogLevel::Debug => 4,
    }
}

#[cfg(feature = "async")]
async fn level_priority(level: LogLevel) -> u8 {
    match level {
        LogLevel::Error => 1,
        LogLevel::Warn => 2,
        LogLevel::Info => 3,
        LogLevel::Debug => 4,
    }
}

#[cfg(feature = "async")]
impl Log {
    pub async fn set_up_logger(level: LogLevel) {
        *LOG_LEVEL.write().await = level;
    }

    pub async fn log_with_level(level: LogLevel, message: &str) {
        if level_priority(level).await <= level_priority(get_level().await).await {
            let log = Logger {
                message: message.to_string(),
                level,
                time: SystemTime::now(),
            };

            LOGS.write().await.push(log);
        }
    }

    pub async fn log(message: &str) {
        Self::log_with_level(get_level().await, message).await;
    }

    pub async fn log_info(message: &str) {
        Self::log_with_level(LogLevel::Info, message).await;
    }

    pub async fn log_debug(message: &str) {
        Self::log_with_level(LogLevel::Debug, message).await;
    }

    pub async fn log_error(message: &str) {
        Self::log_with_level(LogLevel::Error, message).await;
    }

    pub async fn log_warn(message: &str) {
        Self::log_with_level(LogLevel::Warn, message).await;
    }

    pub async fn get_logs() -> Vec<String> {
        LOGS.read()
            .await
            .iter()
            .map(|log| {
                let since_unix = log
                    .time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0));
                format!(
                    "[{:?}] @ {}s → {}",
                    log.level,
                    since_unix.as_secs(),
                    log.message
                )
            })
            .collect()
    }

    pub async fn print_logs() {
        for log in Self::get_logs().await {
            println!("{}", log);
        }
    }

    pub async fn clear() {
        LOGS.write().await.clear();
    }
}

#[cfg(not(feature = "async"))]
impl Log {
    pub fn set_up_logger(level: LogLevel) {
        *LOG_LEVEL.write().unwrap() = level;
    }

    pub fn log_with_level(level: LogLevel, message: &str) {
        if level_priority(level) <= level_priority(get_level()) {
            let log = Logger {
                message: message.to_string(),
                level,
                time: SystemTime::now(),
            };

            LOGS.write().unwrap().push(log);
        }
    }

    pub fn log(message: &str) {
        Self::log_with_level(get_level(), message);
    }

    pub fn log_info(message: &str) {
        Self::log_with_level(LogLevel::Info, message);
    }

    pub fn log_debug(message: &str) {
        Self::log_with_level(LogLevel::Debug, message);
    }

    pub fn log_error(message: &str) {
        Self::log_with_level(LogLevel::Error, message);
    }

    pub fn log_warn(message: &str) {
        Self::log_with_level(LogLevel::Warn, message);
    }

    pub fn get_logs() -> Vec<String> {
        LOGS.read()
            .unwrap()
            .iter()
            .map(|log| {
                let since_unix = log
                    .time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0));
                format!(
                    "[{:?}] @ {}s → {}",
                    log.level,
                    since_unix.as_secs(),
                    log.message
                )
            })
            .collect()
    }

    pub fn print_logs() {
        for log in Self::get_logs() {
            println!("{}", log);
        }
    }

    pub fn clear() {
        LOGS.write().unwrap().clear();
    }
}

#[cfg(feature = "async")]
#[async_trait(?Send)]
pub trait Loggable: Sized + Send + 'static {
    async fn log(&self);
    async fn log_info(&self);
    async fn log_error(&self);
    async fn log_debug(&self);
}

#[cfg(not(feature = "async"))]
pub trait Loggable {
    fn log(self);
    fn log_info(self);
    fn log_error(self);
    fn log_debug(self);
}

#[cfg(feature = "async")]
#[async_trait(?Send)]
impl Loggable for String {
    async fn log(&self) {
        Log::log(self).await
    }
    async fn log_info(&self) {
        Log::log_info(self).await
    }
    async fn log_error(&self) {
        Log::log_error(self).await
    }
    async fn log_debug(&self) {
        Log::log_debug(self).await
    }
}

#[cfg(not(feature = "async"))]
impl Loggable for &str {
    fn log(self) {
        Log::log(self);
    }
    fn log_info(self) {
        Log::log_info(self);
    }
    fn log_error(self) {
        Log::log_error(self);
    }
    fn log_debug(self) {
        Log::log_debug(self);
    }
}

#[cfg(not(feature = "async"))]
impl Loggable for String {
    fn log(self) {
        self.as_str().log()
    }
    fn log_info(self) {
        self.as_str().log_info()
    }
    fn log_error(self) {
        self.as_str().log_error()
    }
    fn log_debug(self) {
        self.as_str().log_debug()
    }
}
