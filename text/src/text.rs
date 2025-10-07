// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput,
};
use pups_core::Input;
use std::marker::PhantomData;

/// UTF-8 text that can be consumed by parsers
pub struct Text<T = char>
where
    T: Character
{
    /// The buffer that stores the `Text`
    buffer: String,
    /// The byte offset in the buffer that represents the start of the `Text`
    byte_offset: usize,
    /// Phantom data used to allow a generic `Character` type
    _phantom_data: PhantomData<T>
}

impl<T> Input for Text<T>
where
    T: Character
{

    type Item = T;

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

    fn peek(&self) -> Option<Self::Item> { T::next_in(&self.buffer[self.byte_offset..]) }

}

impl<T> TextInput for Text<T>
where
    T: Character
{
    fn starts_with(&self, string: &str) -> bool { self.buffer[self.byte_offset..].starts_with(string) }

    fn skip_bytes(&mut self, byte_count: usize) { self.set_cursor(self.byte_offset + byte_count) }

}