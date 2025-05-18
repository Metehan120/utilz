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
