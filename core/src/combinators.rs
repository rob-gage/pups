// Copyright Rob Gage 2025

use crate::{
    Input,
    parsers::*,
    Check,
    ModeResult,
    Parse
};

/// Methods implemented for all parsers that allow easy construction of parser combinators
pub trait Combinators<'a, O, E, M, I>
where
    Self: Parser<'a, O, E, M, I> + Sized + 'a,
    I: Input<'a>
{

    /// Boxes a parser, performing type erasure so it can be used in combinators like `choice`
    fn boxed(self) -> impl Parser<'a, O, E, M, I>
    { boxed(self) }

    /// If necessary, runs a fallback parser to recover from the failure of the original parser
    /// while preserving all messages from the original parser
    fn catch<P>(
        self,
        fallback: P
    ) -> impl Parser<'a, O, E, M, I>
    where
        P: Parser<'a, O, E, M, I>
    { recoverable(self, fallback) }

    /// Replaces a parser's output
    fn emit<_O>(
        self,
        output: _O
    ) -> impl Parser<'a, _O, E, M, I>
    where
        _O: Clone,
    { emitting(self, output) }

    /// Applies a parser optionally, returning `None` instead of an error if it fails
    fn or_not(self) -> impl Parser<'a, Option<O>, E, M, I>
    { optional(self) }

    /// Maps a parser's output to another type using a function
    fn map<_O>(
        self,
        f: impl Fn(O) -> _O + Clone
    ) -> impl Parser<'a, _O, E, M, I>
    { mapped(self, f) }

    /// Maps a parser's error to another type using a function
    fn map_error<_E>(
        self,
        f: impl Fn(E) -> _E + Clone
    ) -> impl Parser<'a, O, _E, M, I>
    { mapped_error(self, f) }

    /// Maps a parser's messages to another type using a function
    fn map_messages<_M>(
        self,
        f: impl Fn(M) -> _M + Clone
    ) -> impl Parser<'a, O, E, _M, I>
    { mapped_messages(self, f) }

    /// Applies another parser in sequence after this one, and returns both results as a tuple
    fn then<P, _O>(
        self,
        next: P
    ) -> impl Parser<'a, (O, _O), E, M, I>
    where
        P: Parser<'a, _O, E, M, I>
    { sequenced(self, next) }

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

impl<'a, O, E, M, I, P> Combinators<'a, O, E, M, I> for P
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I> + Sized + 'a,
{ }