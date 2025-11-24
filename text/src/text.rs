// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput,
};
use pups_core::Input;
use std::{
    cell::UnsafeCell,
    marker::PhantomData,
};

/// UTF-8 text that can be consumed by parsers
pub struct Text {
    /// The buffer that stores the `Text`
    buffer: String,
    /// The byte offset in the buffer that represents the start of the `Text`
    byte_offset: UnsafeCell<usize>,
}

impl<'a> Text {

    /// Creates a new `Text` from a `&str`
    pub fn from_string(string: &'a str) -> Self {
        Self { buffer: string.to_string(), byte_offset: UnsafeCell::new(0) }
    }

}

impl<'a> Input<'a> for Text {

    type Item = char;

    type Slice = &'a str;

    fn advance(&self) {
        if let Some (character) = self.peek() {
            unsafe { *self.byte_offset.get() += character.length() };
        }
    }

    fn peek(&self) -> Option<Self::Item> {
        let byte_offset: usize = unsafe { *self.byte_offset.get() };
        self.buffer[byte_offset..].chars().next()
    }

    fn slice(&'a self, start: usize, end: usize) -> &'a str { &self.buffer[start..end] }

    fn move_cursor(&self, cursor: usize) { unsafe { *self.byte_offset.get() = cursor } }

    fn store_cursor(&self) -> usize { unsafe { *self.byte_offset.get() } }

}

impl<'a> TextInput for Text {

    fn starts_with(&self, string: &str) -> bool {
        let byte_offset: usize = unsafe { *self.byte_offset.get() };
        self.buffer[byte_offset..].starts_with(string)
    }

    fn skip_bytes(&self, byte_count: usize) { unsafe { *self.byte_offset.get() += byte_count; } }

}