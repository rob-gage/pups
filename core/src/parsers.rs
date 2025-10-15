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
use mapped::{
    OutputMapped,
    ErrorMapped,
    MessagesMapped,
};
use nothing::Nothing;
use optional::Optional;
use recoverable::Recoverable;
use sequenced::Sequenced;


/// Implementors can be parsed from an input type
pub trait Parser<'a, O, E, M, I>
where
    Self: Sized,
    I: Input<'a>
{

    // PARSER IMPLEMENTATION

    /// Applies a parser
    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<O, E, M, _Mode>;

    /// Checks that the input matches this parser, and consumes matched input
    fn check(
        &self,
        input: &'a I,
    ) -> ModeResult<O, E, M, Check>
    { self.apply::<Check>(input) }

    /// Checks that the input matches this parser, and consumes matched input
    fn parse(
        &self,
        input: &'a I,
    ) -> ModeResult<O, E, M, Parse>
    { self.apply::<Parse>(input) }

    // COMBINATOR METHODS

    /// If necessary, runs a fallback parser to recover from the failure of the original parser
    /// while preserving all messages from the original parser
    fn catch<P>(
        self,
        fallback: P
    ) -> impl Parser<'a, O, E, M, I>
    where
        P: Parser<'a, O, E, M, I>
    { Recoverable { fallback, parser: self } }

    /// Tries another parser if this one fails
    fn or<P, _E>(
        self,
        alternate: P
    ) -> impl Parser<'a, O, (E, _E), M, I>
    where
        P: Parser<'a, O, _E, M, I>
    {  Choice { primary: self, alternate } }

    /// Applies a parser optionally, returning `None` instead of an error if it fails
    fn or_not(self) -> impl Parser<'a, Option<O>, E, M, I>
    { optional(self) }

    /// Maps a parser's output to another type using a function
    fn map<_O>(
        self,
        f: impl Fn(O) -> _O + Clone
    ) -> impl Parser<'a, _O, E, M, I>
    { OutputMapped { parser: self, function: f, _phantom: PhantomData } }

    /// Maps a parser's error to another type using a function
    fn map_error<_E>(
        self,
        f: impl Fn(E) -> _E + Clone
    ) -> impl Parser<'a, O, _E, M, I>
    { ErrorMapped { parser: self, function: f, _phantom: PhantomData } }

    /// Maps a parser's messages to another type using a function
    fn map_messages<_M>(
        self,
        f: impl Fn(M) -> _M + Clone
    ) -> impl Parser<'a, O, E, _M, I>
    { MessagesMapped { parser: self, function: f, _phantom: PhantomData } }

    /// Applies another parser in sequence after this one, and returns both results as a tuple
    fn then<P, _O>(
        self,
        next: P
    ) -> impl Parser<'a, (O, _O), E, M, I>
    where
        P: Parser<'a, _O, E, M, I>
    { Sequenced { head: self, tail: next } }

    /// Applies another parser in sequence after this one, but ignores its result
    fn then_ignore<P, _O>(
        self,
        next: P
    ) -> impl Parser<'a, O, E, M, I>
    where
        P: Parser<'a, _O, E, M, I>
    { terminated(self, next) }

    /// Ignores this parser's result and then applies another
    fn ignore_then<P, _O>(
        self,
        next: P
    ) -> impl Parser<'a, _O, E, M, I>
    where
        P: Parser<'a, _O, E, M, I>
    { preceded(self, next) }

}


impl<'a, O, E, M, F, I> Parser<'a, O, E, M, I> for F
where
    F: Fn(&'a I) -> ModeResult<O, E, M, Parse>,
    I: Input<'a> +'a
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
        _Mode::apply_parser(self, input)
    }

    fn check(&self, input: &'a I) -> ModeResult<O, E, M, Check> {
        if (self)(input).is_success() {
            ModeResult::Success((), ())
        } else { ModeResult::Failure((), ()) }
    }

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
{ OutputMapped {
    parser: Sequenced { head: prefix, tail: parser },
    function: |(_, output)| output,
    _phantom: PhantomData
} }

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

/// Applies a parser followed by an ignored terminator parser
pub const fn terminated<'a, O1, O2, E, M, I, P1, P2>(
    parser: P1,
    terminator: P2
) -> impl Parser<'a, O1, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{
    OutputMapped {
        parser: Sequenced { head: parser, tail: terminator },
        function: |(output, _)| output,
        _phantom: PhantomData,
    }
}