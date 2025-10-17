// Copyright Rob Gage 2025

mod choice;
mod end;
mod first_match;
mod iterated;
mod mapped;
mod nothing;
mod optional;
mod recoverable;
mod sequenced;
mod mapped_messages;
mod mapped_error;

use crate::{
    Check,
    Input,
    Mode,
    Parse,
    ModeResult,
};
use std::marker::PhantomData;

use choice::Choice;
use end::End;
use first_match::FirstMatch;
use iterated::Iterated;
use mapped::Mapped;
use mapped_error::MappedError;
use mapped_messages::MappedMessages;
use nothing::Nothing;
use optional::Optional;
use recoverable::Recoverable;
use sequenced::Sequenced;

/// Implementors can be parsed from an input type
pub trait Parser<'a, O, E, M, I>
where
    I: Input<'a>
{

    /// Applies a parser
    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<O, E, M, _Mode>
    where
        Self: Sized;

    /// Checks input, returning a boolean if it matches this parser
    fn check(&self, input: &'a I) -> bool;

    /// Parses input, returning a fully detailed result
    fn parse(&self, input: &'a I) -> ModeResult<O, E, M, Parse>;

}

impl<'a, O, E, M, F, I> Parser<'a, O, E, M, I> for F
where
    F: Fn(&'a I) -> ModeResult<O, E, M, Parse>,
    I: Input<'a> +'a
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
        _Mode::apply_parser(self, input)
    }

    fn check(&self, input: &'a I) -> bool { self.parse(input).is_success() }

    fn parse(&self, input: &'a I) -> ModeResult<O, E, M, Parse> { self(input) }

}

/// Applies a parser preceded by an ignored prefix parser, and followed by an ignored terminator
/// parser
pub fn delimited<'a, O1, O2, O3, E, M, I, P1, P2, P3>(
    prefix: P1,
    parser: P2,
    terminator: P3,
) -> impl Parser<'a, O2, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
    P3: Parser<'a, O3, E, M, I>,
{ preceded(prefix, terminated(parser, terminator)) }

/// Matches the end of the provided input
pub const fn end<'a, I>() -> impl Parser<'a, (), (), (), I>
where
    I: Input<'a>,
{ End }

/// Parses absolutely nothing
pub const fn nothing<'a, E, M, I>() -> impl Parser<'a, (), E, M, I>
where
    I: Input<'a>,
{ Nothing (PhantomData) }

/// Iterates application of a parser
pub const fn many<'a, O, E, M, I, P>(
    parser: P,
) -> impl Parser<'a, Vec<O>, E, M, I>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{ separated(parser, nothing()) }

/// Maps a parser's output to another type using a function
pub const fn mapped<'a, OA, OB, E, M, I>(
    parser: impl Parser<'a, OA, E, M, I>,
    function: impl Fn(OA) -> OB + Clone
) -> impl Parser<'a, OB, E, M, I>
where
    I: Input<'a>,
{ Mapped { parser, function, _phantom: PhantomData } }

/// Maps a parser's output to another type using a function
pub const fn mapped_errors<'a, O, EA, EB, M, I>(
    parser: impl Parser<'a, O, EA, M, I>,
    function: impl Fn(EA) -> EB + Clone
) -> impl Parser<'a, O, EB, M, I>
where
    I: Input<'a>,
{ MappedError { parser, function, _phantom: PhantomData } }

/// Maps a parser's output to another type using a function
pub const fn mapped_messages<'a, O, E, MA, MB, I>(
    parser: impl Parser<'a, O, E, MA, I>,
    function: impl Fn(MA) -> MB + Clone
) -> impl Parser<'a, O, E, MB, I>
where
    I: Input<'a>,
{ MappedMessages { parser, function, _phantom: PhantomData } }

/// Optionally applies a parser, converting a failure into `Option::None`
pub const fn optional<'a, O, E, M, I, P>(
    parser: P,
) -> impl Parser<'a, Option<O>, E, M, I>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{ Optional (parser) }

/// Consumes input until a parser can be applied successfully or there is no input left
pub const fn seek<'a, O, E, M, I, P>(
    parser: P,
) -> impl Parser<'a, Option<O>, E, M, I>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{ FirstMatch (parser) }

/// Applies a parser after an ignored prefix parser
pub const fn preceded<'a, O1, O2, E,  M, I, P1, P2>(
    prefix: P1,
    parser: P2
) -> impl Parser<'a, O2, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{ mapped(Sequenced { head: prefix, tail: parser }, |(_, output)| output) }

/// Applies a parser, but uses another one to recover if the first fails, keeping messages from both
pub const fn recoverable<'a, O, E, M, I, P1, P2>(
    parser: P1,
    fallback: P2
) -> impl Parser<'a, O, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O, E, M, I>,
    P2: Parser<'a, O, E, M, I>,
{ Recoverable { fallback, parser } }

/// Iterates application of a parser separated by application of another parser
pub const fn separated<'a, E, I, M, O1, O2, P1, P2>(
    parser: P1,
    separator: P2
) -> impl Parser<'a, Vec<O1>, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{ Iterated { maximum: None, minimum: 0, parser, separator, _phantom: PhantomData } }

/// Applies a parser followed by another parser, and returns the outputs as a tuple
pub const fn sequenced<'a, O1, O2, E, M, I, P1, P2>(
    first: P1,
    second: P2
) -> impl Parser<'a, (O1, O2), E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{ Sequenced { head: first, tail: second } }

/// Applies a parser followed by an ignored terminator parser
pub const fn terminated<'a, O1, O2, E, M, I, P1, P2>(
    parser: P1,
    terminator: P2
) -> impl Parser<'a, O1, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{ mapped(Sequenced { head: parser, tail: terminator }, |(output, _)| output) }