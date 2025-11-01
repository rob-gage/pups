// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput,
};
use pups_core::Input;
use std::marker::PhantomData;

/// UTF-8 text that can be consumed by parsers
pub struct Text {
    /// The buffer that stores the `Text`
    buffer: String,
    /// The byte offset in the buffer that represents the start of the `Text`
    byte_offset: usize,
}

impl<'a> Text {

    /// Creates a new `Text` from a `&str`
    pub fn from_string(string: &'a str) -> Self {
        Self { buffer: string.to_string(), byte_offset: 0 }
    }

}

impl<'a> Input<'a> for Text {

    type Item = char;

    type Slice = &'a str;

    fn advance(&self) {
        if let Some (character) = self.peek() {
            unsafe {
                let mutable: *mut Self = self as *const Self as *mut Self;
                (*mutable).byte_offset += character.length();
            }
        }
    }

    fn peek(&self) -> Option<Self::Item> { self.buffer[self.byte_offset..].chars().next() }

    fn slice(&'a self, start: usize, end: usize) -> &'a str { &self.buffer[start..end] }

    fn move_cursor(&self, cursor: usize) {
        unsafe {
            let mutable: *mut Self = self as *const Self as *mut Self;
            (*mutable).byte_offset = cursor;
        }
    }

    fn store_cursor(&self) -> usize { self.byte_offset }

}

impl<'a> TextInput for Text {

    fn starts_with(&self, string: &str) -> bool
    { self.buffer[self.byte_offset..].starts_with(string) }

    fn skip_bytes(&self, byte_count: usize) {
        unsafe {
            let mutable: *mut Self = self as *const Self as *mut Self;
            (*mutable).byte_offset += byte_count;
        }
    }

}