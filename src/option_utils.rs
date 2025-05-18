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
