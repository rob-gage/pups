// Copyright Rob Gage 2025

use crate::Character;
use pups_core::Input;

/// Parses whitespace
pub fn whitespace<I, T>(input: &mut I) -> Result<String, ()> where
    I: Input<Item = T>,
    T: Character,
{
    let mut space: String = String::new();
    while let Some (character) = input.peek() {
        if character.is_whitespace() {
            character.write(&mut space);
            input.advance();
        } else { break; }
    }
    if space.is_empty() { Err (()) } else { Ok (space) }
}