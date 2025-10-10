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

    fn advance(&mut self) {
        if let Some (character) = self.peek() {
            self.byte_offset += character.length();
        }
    }

    fn save(&self) -> usize { self.byte_offset }

    fn restore(&mut self, cursor: usize) { self.byte_offset = cursor; }

    fn next(&mut self) -> Option<Self::Item> {
        if let Some (character) = self.peek() {
            self.byte_offset += character.length();
            Some (character)
        } else { None }
    }

    fn peek(&self) -> Option<Self::Item> { char::next_in(&self.buffer[self.byte_offset..]) }

}

impl<'a> TextInput for Text<'a> {

    fn starts_with(&self, string: &str) -> bool
    { self.buffer[self.byte_offset..].starts_with(string) }

    fn skip_bytes(&mut self, byte_count: usize) { self.restore(self.byte_offset + byte_count) }

}