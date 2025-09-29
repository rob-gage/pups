// Copyright Rob Gage 2025

mod horizontal_space;
mod newline;
mod token;
mod whitespace;

use crate::TextInput;
use pups_core::{
    Input,
    Parser,
};
use token::Token;

/// Parses a given lexeme from text
pub const fn token<I>(
    lexeme: &'static str
) -> impl Parser<I, Output = String, Error = (), Message = ()>
where
    I: Input + TextInput,
{ Token (lexeme) }
