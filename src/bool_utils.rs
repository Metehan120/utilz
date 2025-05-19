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

pub trait IfUtils<T: PartialEq> {
    /// Returns true if `self == other`.
    fn if_eq<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X>;

    /// Returns true if `self != other`.
    fn if_not_eq<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X>;
}

/// Conditional evaluation based on equality.
impl<T: PartialEq> IfUtils<T> for T {
    // If `self == other`, runs `f()` and returns Some.
    fn if_eq<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X> {
        if self == other { Some(f()) } else { None }
    }

    /// If `self != other`, runs `f()` and returns Some.
    fn if_not_eq<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X> {
        if self != other { Some(f()) } else { None }
    }
}

pub trait IfOrdUtils<T> {
    fn if_gt<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X>;
    fn if_lt<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X>;
    fn if_gte<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X>;
    fn if_lte<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X>;
    fn if_between<X, F: FnOnce() -> X>(&self, low: &T, high: &T, f: F) -> Option<X>;
    fn if_between_inclusive<X, F: FnOnce() -> X>(&self, low: &T, high: &T, f: F) -> Option<X>;
}

impl<T> IfOrdUtils<T> for T
where
    T: PartialOrd,
{
    fn if_gt<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X> {
        if self > other { Some(f()) } else { None }
    }

    fn if_lt<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X> {
        if self < other { Some(f()) } else { None }
    }
    fn if_gte<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X> {
        if self >= other { Some(f()) } else { None }
    }
    fn if_lte<X, F: FnOnce() -> X>(&self, other: &T, f: F) -> Option<X> {
        if self <= other { Some(f()) } else { None }
    }
    fn if_between<X, F: FnOnce() -> X>(&self, low: &T, high: &T, f: F) -> Option<X> {
        if self > low && self < high {
            Some(f())
        } else {
            None
        }
    }
    fn if_between_inclusive<X, F: FnOnce() -> X>(&self, low: &T, high: &T, f: F) -> Option<X> {
        if self >= low && self <= high {
            Some(f())
        } else {
            None
        }
    }
}
