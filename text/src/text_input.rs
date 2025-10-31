// Copyright Rob Gage 2025

/// Represents text-based input that can be consumed by parsers
pub trait TextInput {

    /// Returns true if the `TextInput` starts with a given string
    fn starts_with(&self, string: &str) -> bool;

    /// Skips past a given number of bytes in the `TextInput`
    fn skip_bytes(&self, count: usize);

}