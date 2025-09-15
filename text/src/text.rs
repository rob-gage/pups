// Copyright Rob Gage 2025

use crate::Character;
use pups::Input;
use std::marker::PhantomData;

/// UTF-8 text that can be consumed by parsers
pub struct Text<T = char> where
    T: Character
{
    /// The buffer that stores the `Text`
    buffer: String,
    /// The byte offset in the buffer that represents the start of the `Text`
    cursor: usize,
    /// Phantom data used to allow a generic `Character` type
    _phantom_data: PhantomData<T>
}

impl<T> Input for Text<T> where
    T: Character
{

    type Item = T;

    fn cursor(&self) -> usize { self.cursor }

    fn next(&mut self) -> Option<Self::Item> {
        if let Some (character) = self.peek() {
            self.cursor += character.length();
            Some (character)
        } else { None }
    }

    fn peek(&self) -> Option<Self::Item> { T::next(&self.buffer[self.cursor..]) }

    fn set_cursor(&mut self, cursor: usize) { self.cursor = cursor; }

}