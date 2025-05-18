//! # Utilz
//!
//! **Ergonomic utility traits for Rust with zero dependencies by default.**
//!
//! `utilz` provides a curated set of extension traits for common Rust types like
//! `Option`, `Result`, `Vec`, `bool`, `&str`, `HashMap`, and more — to help you
//! write cleaner, shorter, and more expressive code.
//!
//! ---
//!
//! ## Highlights
//!
//! - **`Log`** – Simple in-memory logger with optional async support
//!   – `.log_info()`, `.log_warn()`, `.print_logs()`, `.set_up_logger()`, `.clear()`
//!
//! - **`OptionUtils`** – More ergonomic handling of `Option<T>`
//!   – `.if_some()`, `.or_default_with()`
//!
//! - **`ResultUtils`** – Sugar methods for `Result<T, E>`
//!   – `.if_ok()`, `.if_err()`, `.unwrap_or_exit()`
//!
//! - **`BoolUtils`** – Conditionals made fancy
//!   – `.toggle()`, `.not()`, `.then_val()`, `.if_true()`, `.if_false()`
//!
//! - **`VecUtils`** – Push conditionally into vectors
//!   – `.push_if()`, `.push_if_with()`
//!
//! - **`StrUtils`** – Extensions for `&str`
//!   – `.contains_all()`, `.contains_any()`, `.to_title_case()`
//!
//! - **`MapUtils`** – `HashMap` helpers
//!   – `.insert_if()`, `.get_or()`
//!
//! - **`MemUtils`** – Reflection-like methods
//!   – `.type_name()`, `.mem_size()`, `.view()`
//!
//! - **`IdentityUtils`** – Tap-style chaining
//!   – `.tap()`
//!
//! - **`PanicUtils`** – Fatal exit helpers
//!   – `.unwrap_or_exit()`
//!
//! - **`DurationUtils`** – Duration formatting
//!   – `.pretty()` → `"1h 2m 3s"`
//!
//! - **`ConvertUtils`** – Easy type conversions with `TryFrom`
//!   – `.to()`, `.to_or()`, `.to_result()`
//!
//! - **`ClampUtils`** – Range limiting for numbers
//!   – `.clamp_to(min, max)`
//!
//! - **`NumberUtils`** – Integer extensions
//!   – `.is_even()`, `.is_odd()`
//!
//! - **`IteratorUtils`** – Fallback logic for iterators
//!   – `.find_map_or(f, fallback)`
//!
//! ---
//!
//! ## ✨ Quick Example
//!
//! ```rust
//! use utilz_rs::*;
//!
//! let value = Some("hi");
//! value.if_some(|v| println!("Got: {v}"));
//!
//! let mut enabled = true;
//! enabled.toggle();
//!
//! let name = "hello world";
//! assert!(name.contains_all(["hello", "world"]));
//!
//! let duration = std::time::Duration::from_secs(3666);
//! println!("{}", duration.pretty()); // → "1h 1m 6s"
//! ```
//!
//! ---
//!
//! ## Philosophy
//!
//! - ✅ 100% Rust standard library
//! - 🔧 Zero dependencies by default
//! - 🔌 Async logging via optional feature flag
//! - 🙌 Opt-in trait imports: use only what you need
//!
//! _Use what you want, ignore the rest. No macros. No surprises._

#[cfg(not(feature = "async"))]
use std::sync::RwLock;
use std::time::UNIX_EPOCH;

use std::{
    any::type_name,
    collections::HashMap,
    hash::Hash,
    time::{Duration, SystemTime},
};

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

/// Provides sugar methods for comparing values.
pub trait EqUtils<T: PartialEq> {
    /// Returns true if `self == other`.
    fn eq_to(&self, other: &T) -> bool;

    /// Returns true if `self != other`.
    fn not_eq_to(&self, other: &T) -> bool;
}

impl<T: PartialEq> EqUtils<T> for T {
    fn eq_to(&self, other: &T) -> bool {
        self == other
    }
    fn not_eq_to(&self, other: &T) -> bool {
        self != other
    }
}

/// Extension methods for `Option<T>`.
pub trait OptionUtils<T> {
    /// Returns the value inside `Some`, or the fallback if `None`.
    fn or_default_with(self, fallback: T) -> T;

    /// Executes a closure if the `Option` is `Some`.
    ///
    /// Returns the same `Option` back.
    #[must_use]
    fn if_some<F: FnOnce(&T)>(self, f: F) -> Option<T>;
}

impl<T> OptionUtils<T> for Option<T> {
    fn or_default_with(self, fallback: T) -> T {
        self.unwrap_or(fallback)
    }

    fn if_some<F: FnOnce(&T)>(self, f: F) -> Option<T> {
        if let Some(ref val) = self {
            f(val);
        }
        self
    }
}

/// Extra methods for string slices (`&str`).
pub trait StrUtils {
    /// Returns `true` if all strings in the iterator exist in the main string.
    fn contains_all<'a, I>(&self, parts: I) -> bool
    where
        I: IntoIterator<Item = &'a str>;

    /// Returns `true` if any of the strings in the iterator exist in the main string.
    fn contains_any<'a, I>(&self, parts: I) -> bool
    where
        I: IntoIterator<Item = &'a str>;

    /// Returns a new string with the first letter capitalized.
    fn to_title_case(&self) -> String;
}

impl StrUtils for str {
    fn contains_all<'a, I>(&self, parts: I) -> bool
    where
        I: IntoIterator<Item = &'a str>,
    {
        parts.into_iter().all(|part| self.contains(part))
    }
    fn contains_any<'a, I>(&self, parts: I) -> bool
    where
        I: IntoIterator<Item = &'a str>,
    {
        parts.into_iter().any(|part| self.contains(part))
    }
    fn to_title_case(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

/// Reflection helpers: type name and memory size.
pub trait MemUtils {
    /// Returns the type name of `self`.
    fn type_name(&self) -> &'static str;

    /// Returns memory size in bytes of the type.
    fn mem_size(&self) -> usize;

    /// Prints type and memory size to stdout.
    fn view(&self);
}

impl<T> MemUtils for T {
    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
    fn mem_size(&self) -> usize {
        std::mem::size_of::<T>()
    }
    fn view(&self) {
        println!(
            "[view] Type: {}, Size: {} bytes",
            self.type_name(),
            self.mem_size()
        );
    }
}

pub trait ConvertUtils: Sized {
    fn to<T: TryFrom<Self>>(self) -> Option<T>;
    fn to_or<T: TryFrom<Self>>(self, fallback: T) -> T;
    fn to_result<T: TryFrom<Self>>(self) -> Result<T, T::Error>;
}

impl<T> ConvertUtils for T {
    fn to<U: TryFrom<T>>(self) -> Option<U> {
        U::try_from(self).ok()
    }

    fn to_or<U: TryFrom<T>>(self, fallback: U) -> U {
        self.to().unwrap_or(fallback)
    }

    fn to_result<U: TryFrom<T>>(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

pub trait BoolUtils {
    #[must_use]
    fn not(&self) -> bool;
    fn then_val<T>(&self, val: T) -> Option<T>;
    fn if_true<T, F: FnOnce() -> T>(&self, f: F) -> Option<T>;
    fn if_false<T, F: FnOnce() -> T>(&self, f: F) -> Option<T>;
    fn toggle(&mut self);
}

impl BoolUtils for bool {
    fn not(&self) -> bool {
        !self
    }
    fn then_val<T>(&self, val: T) -> Option<T> {
        if *self { Some(val) } else { None }
    }
    fn if_true<T, F: FnOnce() -> T>(&self, f: F) -> Option<T> {
        if *self { Some(f()) } else { None }
    }
    fn if_false<T, F: FnOnce() -> T>(&self, f: F) -> Option<T> {
        if self.not() { Some(f()) } else { None }
    }
    fn toggle(&mut self) {
        *self = !*self;
    }
}

/// Conditional vector push helpers.
pub trait VecUtils<T> {
    /// Pushes the value if `cond` is `true`.
    fn push_if(&mut self, push: T, cond: bool);

    /// Lazily evaluates and pushes the value if `cond` is `true`.
    fn push_if_with<F: FnOnce() -> T>(&mut self, cond: bool, f: F);
}

impl<T> VecUtils<T> for Vec<T> {
    fn push_if(&mut self, push: T, cond: bool) {
        if cond {
            self.push(push);
        }
    }
    fn push_if_with<F: FnOnce() -> T>(&mut self, cond: bool, f: F) {
        if cond {
            self.push(f());
        }
    }
}

pub trait MapUtils<K, V> {
    fn get_or<'a>(&'a self, key: &K, fallback: &'a V) -> &'a V;
    fn insert_if(&mut self, key: K, value: V, cond: bool);
}

impl<K: Eq + Hash, V> MapUtils<K, V> for HashMap<K, V> {
    fn get_or<'a>(&'a self, key: &K, fallback: &'a V) -> &'a V {
        self.get(key).unwrap_or(fallback)
    }

    fn insert_if(&mut self, key: K, value: V, cond: bool) {
        if cond {
            self.insert(key, value);
        }
    }
}

pub trait ResultUtils<T, E> {
    fn if_ok<F: FnOnce(&T)>(self, f: F) -> Self;
    fn if_err<F: FnOnce(&E)>(self, f: F) -> Self;
}

impl<T, E: std::fmt::Debug> ResultUtils<T, E> for Result<T, E> {
    fn if_ok<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Ok(ref val) = self {
            f(val);
        }
        self
    }

    fn if_err<F: FnOnce(&E)>(self, f: F) -> Self {
        if let Err(ref err) = self {
            f(err);
        }
        self
    }
}

/// Pretty-formatting for `Duration`.
pub trait DurationUtils {
    /// Returns a formatted string like `"1h 20m 5s"`.
    fn pretty(&self) -> String;
}

impl DurationUtils for Duration {
    fn pretty(&self) -> String {
        let total_secs = self.as_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        format!("{}h {}m {}s", hours, mins, secs)
    }
}

pub trait IteratorUtils: Iterator + Sized {
    fn find_map_or<T, F: FnMut(Self::Item) -> Option<T>>(self, f: F, fallback: T) -> T;
}

impl<I: Iterator> IteratorUtils for I {
    fn find_map_or<T, F: FnMut(Self::Item) -> Option<T>>(mut self, f: F, fallback: T) -> T {
        self.find_map(f).unwrap_or(fallback)
    }
}

pub trait IdentityUtils: Sized {
    fn tap<F: FnOnce(&Self)>(self, f: F) -> Self;
}

impl<T> IdentityUtils for T {
    fn tap<F: FnOnce(&Self)>(self, f: F) -> Self {
        f(&self);
        self
    }
}

/// Helpers to panic or exit cleanly with messages.
pub trait PanicUtils<T> {
    /// If `None` or `Err`, logs the message and exits the program.
    fn unwrap_or_exit(self, msg: &str) -> T;
}

impl<T> PanicUtils<T> for Option<T> {
    fn unwrap_or_exit(self, msg: &str) -> T {
        self.unwrap_or_else(|| {
            eprintln!("[FATAL]: {}", msg);
            std::process::exit(1);
        })
    }
}

impl<T, U> PanicUtils<T> for Result<T, U> {
    fn unwrap_or_exit(self, msg: &str) -> T {
        self.unwrap_or_else(|_| {
            eprintln!("[FATAL]: {}", msg);
            std::process::exit(1);
        })
    }
}

pub trait ClampUtils {
    fn clamp_to(self, min: Self, max: Self) -> Self;
}
impl ClampUtils for i32 {
    fn clamp_to(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
}

pub trait NumberUtils {
    #[must_use]
    fn is_even(&self) -> bool;
    #[must_use]
    fn is_odd(&self) -> bool;
}

impl NumberUtils for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
    fn is_odd(&self) -> bool {
        self % 2 != 0
    }
}
