// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput,
};
use pups_core::Input;
use std::marker::PhantomData;

/// UTF-8 text that can be consumed by parsers
pub struct Text<'input> {
    /// The buffer that stores the `Text`
    buffer: &'input str,
    /// The byte offset in the buffer that represents the start of the `Text`
    byte_offset: usize,
}

impl<'a> Text<'a> {

    /// Creates a new `Text` from a `&str`
    fn from_string(string: &'a str) -> Self {
        Self { buffer: string, byte_offset: 0 }
    }

}

impl<'a> Input<'a> for Text<'a> {

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

    fn consume(&'a self, length: usize) -> Option<&'a str> {
        let start: usize = self.save();
        let mut slice_byte_count: usize = 0;
        for _ in 0..length {
            if let Some(character) = self.next() {
                slice_byte_count += character.length();
            } else { return None; }
        }
        let slice: &'a str = &self.buffer[start..start + slice_byte_count];
        Some (slice)
    }

    fn next(&self) -> Option<Self::Item> {
        if let Some (character) = self.peek() {
            unsafe {
                let mutable: *mut Self = self as *const Self as *mut Self;
                (*mutable).byte_offset += character.length();
            }
            Some (character)
        } else { None }
    }

    fn peek(&self) -> Option<Self::Item> {
        char::next_in(&self.buffer[self.byte_offset..])
    }

    fn restore(&self, cursor: usize) {
        unsafe {
            let mutable: *mut Self = self as *const Self as *mut Self;
            (*mutable).byte_offset = cursor;
        }
    }

    fn save(&self) -> usize { self.byte_offset }

}

impl<'a> TextInput for Text<'a> {

    fn starts_with(&self, string: &str) -> bool
    { self.buffer[self.byte_offset..].starts_with(string) }

    fn skip_bytes(&mut self, byte_count: usize) { self.restore(self.byte_offset + byte_count) }

}