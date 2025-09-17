// Copyright Rob Gage 2025

mod delimited;
mod mapped;
mod preceded;
mod terminated;

use crate::{
    Input,
    Parser,
    Sequence,
};

pub use delimited::delimited;
pub use mapped::mapped;
pub use preceded::preceded;
pub use terminated::terminated;


/// Helper methods implemented for all parsers that allows easy construction of combinators
pub trait Combinators<E, I, O> where
    Self: Parser<I, Error = E, Output = O> + Sized,
    I: Input
{

    /// Ignores this parser's result and applies another in sequence after it
    fn ignore_then<P, _O>(self, parser: P) -> impl Parser<I, Error = E, Output = _O> where
        P: Parser<I, Error = E, Output = _O>
    { Sequence { head: self, tail: parser }.map(|(_, output)| output) }

    /// Maps a parser's output to another type using a function
    fn map<F, _O>(self, f: F) -> impl Parser<I, Error = E, Output = _O> where
        F: Fn(O) -> _O
    {
        mapped(self, move |result: Result<O, Vec<E>>| result.map(|output: O| f(output)))
    }

    /// Maps each of a parser's accumulated errors to a new type using a function
    fn map_errors<F, _E>(self, f: F) -> impl Parser<I, Error = _E, Output = O> where
        F: Clone + Fn(E) -> _E
    {
        mapped(self, move |result: Result<O, Vec<E>>| result
            .map_err(|errors: Vec<E>| errors.into_iter().map(f.clone()).collect()))
    }

    /// Applies another parser in sequence after this one
    fn then<P, _O>(self, parser: P) -> impl Parser<I, Error = E, Output = (O, _O)> where
        P: Parser<I, Error = E, Output = _O>
    { Sequence { head: self, tail: parser } }

    /// Applies another parser in sequence after this one, but ignores its result
    fn then_ignore<P, _O>(self, parser: P) -> impl Parser<I, Error = E, Output = O> where
        P: Parser<I, Error = E, Output = _O>
    { Sequence { head: self, tail: parser }.map(|(output, _)| output) }

}


impl<E, I, O, T> Combinators<E, I, O> for T
where
    I: Input,
    T: Parser<I, Error = E, Output = O>
{ }