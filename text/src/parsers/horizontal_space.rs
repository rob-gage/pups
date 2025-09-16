// Copyright Rob Gage 2025

use crate::Character;
use pups::Input;

/// Parses whitespace not counting newlines
pub fn horizontal_space<I, T>(input: &mut I) -> Result<String, ()> where
    I: Input<Item = T>,
    T: Character,
{
    let mut space: String = String::new();
    while let Some (character) = input.peek() {
        if character.is_whitespace() && (!character.is_newline()) {
            character.write(&mut space);
            input.advance();
        } else { break; }
    }
    if space.is_empty() { Err (()) } else { Ok (space) }
}