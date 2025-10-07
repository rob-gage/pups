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

impl<'input> Text<'input> {

    /// Creates a new `Text` from a `&str`
    fn from_string(string: &'input str) -> Self {
        Self { buffer: string, byte_offset: 0 }
    }

}

impl<'input> Input for Text<'input> {

    type Item = char;

    fn advance(&mut self) {
        if let Some (character) = self.peek() {
            self.byte_offset += character.length();
        }
    }

    fn cursor(&self) -> usize { self.byte_offset }

    fn set_cursor(&mut self, cursor: usize) { self.byte_offset = cursor; }

    fn next(&mut self) -> Option<Self::Item> {
        if let Some (character) = self.peek() {
            self.byte_offset += character.length();
            Some (character)
        } else { None }
    }

    fn peek(&self) -> Option<Self::Item> { char::next_in(&self.buffer[self.byte_offset..]) }

}

impl<'input> TextInput for Text<'input> {

    fn starts_with(&self, string: &str) -> bool
    { self.buffer[self.byte_offset..].starts_with(string) }

    fn skip_bytes(&mut self, byte_count: usize) { self.set_cursor(self.byte_offset + byte_count) }

}