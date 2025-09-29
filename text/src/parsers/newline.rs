// Copyright Rob Gage 2025

use crate::Character;
use pups_core::Input;

/// Parses a newline
pub fn newline<I, T>(input: &mut I) -> Result<(), ()> where
    I: Input<Item = T>,
    T: Character,
{
    let character: T = input.peek().ok_or(())?;
    if character.is_newline() { input.advance(); Ok (()) } else { Err (()) }
}