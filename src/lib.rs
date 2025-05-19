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
//! ## Quick Example
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

use std::{any::type_name, collections::HashMap, hash::Hash, time::Duration};

pub mod bool_utils;
pub mod logger;
pub mod option_utils;
pub mod str_utils;

pub mod prelude {
    pub use crate::bool_utils::*;
    pub use crate::logger::Loggable;
    pub use crate::option_utils::*;
    pub use crate::str_utils::*;
    pub use crate::*;
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
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}

impl<T> NumberUtils for T
where
    T: Copy + PartialEq + std::ops::Rem<Output = T> + From<u8>,
{
    fn is_even(&self) -> bool {
        *self % T::from(2u8) == T::from(0u8)
    }
    fn is_odd(&self) -> bool {
        *self % T::from(2u8) != T::from(0u8)
    }
}

pub trait UNumberUtils {
    #[must_use]
    fn is_even(&self) -> bool;
    #[must_use]
    fn is_odd(&self) -> bool;
}
impl UNumberUtils for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
    fn is_odd(&self) -> bool {
        self % 2 != 0
    }
}

pub trait BitwiseUtils<Rhs = Self> {
    fn xor(self, rhs: Rhs) -> Self;
}

impl BitwiseUtils for u32 {
    fn xor(self, rhs: Self) -> Self {
        self ^ rhs
    }
}
