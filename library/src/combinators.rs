// Copyright Rob Gage 2025

mod choice;
mod delimited;
mod mapped;
mod preceded;
mod sequence;
mod terminated;

use choice::Choice;
use crate::{
    Input,
    Parser,
};
use sequence::Sequence;

pub use delimited::delimited;
pub use mapped::mapped;
pub use preceded::preceded;
pub use terminated::terminated;

/// Helper methods implemented for all parsers that allows easy construction of combinators
pub trait Combinators<E, I, O> where
    Self: Parser<I, Error = E, Output = O> + Sized,
    I: Input
{

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

    /// Tries parsers in order until one succeeds
    fn or<P>(self, alternative_parser: P) -> Choice<E, I, O>
    where
        Self: 'static,
        P: Parser<I, Error = E, Output = O> + 'static
    {
        Choice (vec![Box::new(self), Box::new(alternative_parser)])
    }

    /// Applies another parser in sequence
    fn then<P>(self, next_parser: P) -> Sequence<E, I, O>
    where
        Self: 'static,
        P: Parser<I, Error = E, Output = O> + 'static
    {
        Sequence(vec![(Box::new(self), true), (Box::new(next_parser), true)])
    }

    /// Applies another parser in sequence, but ignores its result
    fn then_ignore<P>(self, next_parser: P) -> Sequence<E, I, O>
    where
        Self: 'static,
        P: Parser<I, Error = E, Output = O> + 'static
    {
        Sequence(vec![(Box::new(self), true), (Box::new(next_parser), false)])
    }

}

impl<E, I, O, T> Combinators<E, I, O> for T
where
    I: Input,
    T: Parser<I, Error = E, Output = O>
{ }