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
