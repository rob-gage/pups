// Copyright Rob Gage 2025

mod newline;
mod number;
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
use number::Number;
use token::Token;
use whitespace::Whitespace;
use crate::parsers::unicode_identifier::UnicodeIdentifier;

/// Parses a single newline character
pub const fn newline<'a, C, I>() -> impl Parser<'a, (), (), (), I>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ Newline }

/// Parses a number composed of ASCII decimal digits 0-9
pub const fn number<'a, C, I>(
) -> impl Parser<'a, I::Slice, (), (), I>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ Number }

/// Parses a given lexeme from text
pub const fn token<'a, I>(
    lexeme: &'static str
) -> impl Parser<'a, &'static str, (), (), I>
where
    I: Input<'a> + TextInput,
{ Token (lexeme) }

/// Parses a unicode identifier
pub const fn unicode_identifier<'a, C, I>(
) -> impl Parser<'a, I::Slice, (), (), I>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ UnicodeIdentifier }

/// Parses whitespace
pub const fn whitespace<'a, C, I>(
) -> impl Parser<'a, I::Slice, (), (), I>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ Whitespace }
