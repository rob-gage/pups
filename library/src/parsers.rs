// Copyright Rob Gage 2025

mod choice;
mod mapped;
mod sequenced;

use crate::{
    Input,
    ParseResult::{
        self,
        Failure,
        Success,
    },
};
use choice::Choice;
use mapped::Mapped;
use sequenced::Sequenced;

/// Implementors can be parsed from an input type
pub trait Parser<I> where
    Self: Sized,
    I: Input
{

    // PARSER IMPLEMENTATION

    /// The type for errors that can occur with this parser
    type Error;

    /// The output type that is parsed by this parser
    type Output;

    /// Parses a `Input` to return `Self::Output` or `Vec<Self::Error>`
    fn parse(&self, input: &mut I) -> ParseResult<Self::Output, Self::Error>;

    // COMBINATOR METHODS

    /// Ignores this parser's result and applies another in sequence after it
    fn ignore_then<P, O>(
        self,
        next: P
    ) -> impl Parser<I, Error = Self::Error, Output = O> where
        P: Parser<I, Error = Self::Error, Output = O>
    { preceded(self, next) }

    // /// Tries another parser if this one fails
    // fn or<P>(
    //     self,
    //     alternative: P
    // ) -> impl Parser<I, Error = Self::Error, Output = Self::Error> where
    //     P: Parser<I, Error = Self::Error, Output = Self::Output>;
    // { Choice { alternative, primary: self } }

    /// Try a parser as optional
    fn or_not(
        self
    ) -> impl Parser<I, Error = Self::Error, Output = Option<Self::Output>>
    { optional(self) }

    /// Maps a parser's output to another type using a function
    fn map<F, O>(
        self,
        f: F
    ) -> impl Parser<I, Error = Self::Error, Output = O> where
        F: Clone + Fn(Self::Output) -> O
    {
        mapped(self, move |result| result.map(f.clone()))
    }

    /// Maps each of a parser's accumulated errors to a new type using a function
    fn map_errors<F, E>(
        self,
        f: F
    ) -> impl Parser<I, Error = E, Output = Self::Output> where
        F: Clone + Fn(Self::Error) -> E
    {
        mapped(self, move |result| result.map_errors(f.clone()))
    }

    /// Applies another parser in sequence after this one
    fn then<P, O>(
        self,
        next: P
    ) -> impl Parser<I, Error = Self::Error, Output = (Self::Output, O)> where
        P: Parser<I, Error = Self::Error, Output = O>
    { Sequenced { head: self, tail: next } }

    /// Applies another parser in sequence after this one, but ignores its result
    fn then_ignore<P, O>(
        self,
        next: P
    ) -> impl Parser<I, Error = Self::Error, Output = Self::Output> where
        P: Parser<I, Error = Self::Error, Output = O>
    { terminated(self, next) }

}


/// Applies a parser preceded by an ignored prefix parser, and followed by an ignored terminator
/// parser
pub fn delimited<E, I, O1, O2, O3, P1, P2, P3>(
    prefix: P1,
    parser: P2,
    terminator: P3,
) -> impl Parser<I, Error = E, Output = O2>
where
    I: Input,
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
    P3: Parser<I, Output = O3, Error = E>,
{ preceded(prefix, terminated(parser, terminator)) }


/// Applies a parser that becomes `Some (_)` when it works, and `None` when it doesn't
pub const fn optional<E, I, O, P>(parser: P)-> impl Parser<I, Error = E, Output = Option<O>> where
    I: Input,
    P: Parser<I, Output = O, Error = E>,
{
    mapped(parser, |result| match result {
        Success (output, errors) => Success (Some (output), errors),
        Failure (_) => Success(None, vec![])
    })
}


/// Maps the result type of a parser into a new type using a mapper function
pub const fn mapped<EA, EB, I, M, OA, OB, P>(
    parser: P,
    mapper: M
) -> impl Parser<I, Error = EB, Output = OB> where
    I: Input,
    M: Fn(ParseResult<OA, EA>) -> ParseResult<OB, EB>,
    P: Parser<I, Error = EA, Output = OA>,
{ Mapped { mapper, parser } }


/// Applies a parser after an ignored prefix parser
pub fn preceded<E, I, O1, O2, P1, P2>(
    prefix: P1,
    parser: P2
) -> impl Parser<I, Error = E, Output = O2> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
{
    Sequenced { head: prefix, tail: parser }
        .map(|(_, output)| output)
}


/// Applies a parser followed by an ignored terminator parser
pub fn terminated<E, I, O2, O1, P2, P1>(
    parser: P1,
    terminator: P2
) -> impl Parser<I, Error = E, Output = O1> where
    I: Input,
    P2: Parser<I, Output = O2, Error = E>,
    P1: Parser<I, Output = O1, Error = E>,
{
    Sequenced { head: parser, tail: terminator }
        .map(|(output, _)| output)
}