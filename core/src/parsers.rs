// Copyright Rob Gage 2025

mod choice;
mod end;
mod first_match;
mod iterated;
mod mappers;
mod optional;
mod recoverable;
mod sequenced;

use crate::{
    Check,
    Input,
    Mode,
    Parse,
    ParseResult,
};

use choice::Choice;
use end::End;
use first_match::FirstMatch;
use iterated::Iterated;
use mappers::{
    OutputMapper,
    ErrorMapper,
    MessageMapper,
};
use optional::Optional;
use recoverable::Recoverable;
use sequenced::Sequenced;


/// Implementors can be parsed from an input type
pub trait Parser<I>
where
    Self: Sized,
    I: Input
{

    // PARSER IMPLEMENTATION

    /// The output type that is parsed by this parser
    type Output: Sized;

    /// The type for errors that can occur with this parser
    type Error: Sized;

    /// The type for messages that may be returned by this parser
    type Message: Sized;

    /// Applies a parser
    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<Self::Output, Self::Error, Self::Message, _Mode>;

    /// Checks that the input matches this parser, and consumes matched input
    fn check(
        &self,
        input: &mut I,
    ) -> bool
    { self.apply::<Check>(input).is_success() }

    /// Checks that the input matches this parser, and consumes matched input
    fn parse(
        &self,
        input: &mut I,
    ) -> ParseResult<Self::Output, Self::Error, Self::Message, Parse>
    { self.apply::<Parse>(input) }

    // COMBINATOR METHODS

    /// If necessary, runs a fallback parser to recover from the failure of the original parser
    /// while preserving all messages from the original parser
    fn catch<P>(
        self,
        fallback: P
    ) -> impl Parser<I, Output = Self::Output, Error = Self::Error, Message = Self::Message>
    where
        P: Parser<I, Output = Self::Output, Error = Self::Error, Message = Self::Message>
    { Recoverable { fallback, parser: self } }

    /// Ignores this parser's result and then applies another
    fn ignore_then<P, O>(
        self,
        next: P
    ) -> impl Parser<I, Output = O, Error = Self::Error, Message = Self::Message>
    where
        P: Parser<I, Output = O, Error = Self::Error, Message = Self::Message>
    { preceded(self, next) }

    /// Tries another parser if this one fails
    fn or<P, E>(
        self,
        alternate: P
    ) -> impl Parser<I, Output = Self::Output, Error = (Self::Error, E), Message = Self::Message>
    where
        P: Parser<I, Output = Self::Output, Error = E, Message = Self::Message>
    {  Choice { primary: self, alternate } }

    /// Applies a parser optionally, returning `None` instead of an error if it fails
    fn or_not(
        self
    ) -> impl Parser<I, Output = Option<Self::Output>, Error = Self::Error, Message = Self::Message>
    { optional(self) }

    /// Maps a parser's output to another type using a function
    fn map<O>(
        self,
        f: impl Fn(Self::Output) -> O + Clone
    ) -> impl Parser<I, Output = O, Error = Self::Error, Message = Self::Message> {
        OutputMapper { parser: self, function: f }
    }

    /// Maps a parser's error to another type using a function
    fn map_error<E>(
        self,
        f: impl Fn(Self::Error) -> E + Clone
    ) -> impl Parser<I, Output = Self::Output, Error = E, Message = Self::Message> {
        ErrorMapper { parser: self, function: f }
    }

    /// Maps a parser's messages to another type using a function
    fn map_messages<M>(
        self,
        f: impl Fn(Self::Message) -> M + Clone
    ) -> impl Parser<I, Output = Self::Output, Error = Self::Error, Message = M> {
        MessageMapper { parser: self, function: f }
    }

    /// Applies another parser in sequence after this one, and returns both results as a tuple
    fn then<P, O>(
        self,
        next: P
    ) -> impl Parser<I, Output = (Self::Output, O), Error = Self::Error, Message = Self::Message>
    where
        P: Parser<I, Output = O, Error = Self::Error, Message = Self::Message>
    { Sequenced { head: self, tail: next } }

    /// Applies another parser in sequence after this one, but ignores its result
    fn then_ignore<P, O>(
        self,
        next: P
    ) -> impl Parser<I, Output = Self::Output, Error = Self::Error, Message = Self::Message>
    where
        I: Input,
        P: Parser<I, Output = O, Error = Self::Error, Message = Self::Message>
    { terminated(self, next) }

}


/// Applies a parser preceded by an ignored prefix parser, and followed by an ignored terminator
/// parser
pub fn delimited<E, I, M, O1, O2, O3, P1, P2, P3>(
    prefix: P1,
    parser: P2,
    terminator: P3,
) -> impl Parser<I, Error = E, Output = O2, Message = M>
where
    I: Input,
    P1: Parser<I, Output = O1, Error = E, Message = M>,
    P2: Parser<I, Output = O2, Error = E, Message = M>,
    P3: Parser<I, Output = O3, Error = E, Message = M>,
{ preceded(prefix, terminated(parser, terminator)) }


/// Matches the end of the provided input
pub fn end<O, E, M, I, P>() -> impl Parser<I, Output = (), Error = (), Message = ()>
where
    I: Input,
    P: Parser<I, Output = O, Error = E, Message = M>
{ End }


/// Iterates application of a parser
pub const fn many<E, I, M, O1, O2, P1, P2>(
    parser: P1,
) -> impl Parser<I, Output = Vec<O1>, Error = E, Message = M> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E, Message = M>,
    P2: Parser<I, Output = O2, Error = E, Message = M>,
{ Iterated::<P1, P2> { maximum: None, minimum: 0, parser, separator: None } }


/// Optionally applies a parser, converting a failure into `Option::None`
pub const fn optional<O, E, M, I, P>(
    parser: P,
) -> impl Parser<I, Output = Option<O>, Error = E, Message = M> where
    I: Input,
    P: Parser<I, Output = O, Error = E, Message = M>,
{ Optional (parser) }


/// Consumes input until a parser can be applied successfully or there is no input left
pub const fn seek<O, E, M, I, P>(
    parser: P,
) -> impl Parser<I, Output = Option<O>, Error = E, Message = M> where
    I: Input,
    P: Parser<I, Output = O, Error = E, Message = M>,
{ FirstMatch (parser) }


/// Applies a parser after an ignored prefix parser
pub fn preceded<E, I, M, O1, O2, P1, P2>(
    prefix: P1,
    parser: P2
) -> impl Parser<I, Output = O2, Error = E, Message = M> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E, Message = M>,
    P2: Parser<I, Output = O2, Error = E, Message = M>,
{
    Sequenced { head: prefix, tail: parser }
        .map(|(_, tail)| tail)
}


/// Iterates application of a parser separated by application of another parser
pub const fn separated<E, I, M, O1, O2, P1, P2>(
    parser: P1,
    separator: P2
) -> impl Parser<I, Output = Vec<O1>, Error = E, Message = M> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E, Message = M>,
    P2: Parser<I, Output = O2, Error = E, Message = M>,
{ Iterated { maximum: None, minimum: 0, parser, separator: Some(separator) } }


/// Applies a parser followed by an ignored terminator parser
pub fn terminated<E, I, M, O1, O2, P1, P2>(
    parser: P1,
    terminator: P2
) -> impl Parser<I, Output = O1, Error = E, Message = M> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E, Message = M>,
    P2: Parser<I, Output = O2, Error = E, Message = M>,
{
    Sequenced { head: parser, tail: terminator }
        .map(|(head, _)| head)
}