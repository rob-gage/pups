// Copyright Rob Gage 2025

mod newline;
mod token;
mod whitespace;

use crate::{
    Character,
    TextInput
};
use pups_core::{
    Input,
    Parser,
};
use newline::Newline;
use token::Token;
use whitespace::Whitespace;


/// Parses a single newline character
pub const fn newline<C, I>() -> impl Parser<I, Output = C, Error = (), Message = ()>
where
    C: Character,
    I: Input<Item = C> + TextInput,
{ Newline }


/// Parses a given lexeme from text
pub const fn token<I>(
    lexeme: &'static str
) -> impl Parser<I, Output = String, Error = (), Message = ()>
where
    I: Input + TextInput,
{ Token (lexeme) }


/// Parses whitespace
pub const fn whitespace<C, I>() -> impl Parser<I, Output = Vec<C>, Error = (), Message = ()>
where
    C: Character,
    I: Input<Item = C> + TextInput,
{ Whitespace }
