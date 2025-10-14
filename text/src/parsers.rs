// Copyright Rob Gage 2025

mod newline;
mod token;
mod whitespace;
mod unicode_identifier;

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
use crate::parsers::unicode_identifier::UnicodeIdentifier;

/// Parses a single newline character
pub const fn newline<'a, C, I>() -> impl Parser<'a, I, Output = (), Error = (), Message = ()>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ Newline }


/// Parses a given lexeme from text
pub const fn token<'a, I>(
    lexeme: &'static str
) -> impl Parser<'a, I, Output = &'static str, Error = (), Message = ()>
where
    I: Input<'a> + TextInput,
{ Token (lexeme) }


/// Parses a unicode identifier
pub const fn unicode_identifier<'a, C, I>(
) -> impl Parser<'a, I, Output = I::Slice, Error = (), Message = ()>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ UnicodeIdentifier }


/// Parses whitespace
pub const fn whitespace<'a, C, I>(
) -> impl Parser<'a, I, Output = I::Slice, Error = (), Message = ()>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ Whitespace }
