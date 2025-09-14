// Copyright Rob Gage 2025

mod choice;
mod delimited;
mod map;
mod map_errors;
mod preceded;
mod sequence;
mod terminated;

use choice::Choice;
use crate::{
    Input,
    Parser,
};
use map::Map;
use map_errors::MapErrors;
use sequence::Sequence;

pub use preceded::preceded;
pub use terminated::terminated;

/// Helper methods implemented for all parsers that allows easy construction of combinators
pub trait Combinators<E, I, O> where
    Self: Parser<I, Error = E, Output = O> + Sized,
    I: Input
{

    /// Maps a parser's output to another type using a function
    fn map<F>(self, f: F) -> Map<F, Self> {
        Map { function: f, parser: self }
    }

    /// Maps each of a parser's accumulated errors to a new type using a function
    fn map_errors<F>(self, f: F) -> MapErrors<F, Self> {
        MapErrors { function: f, parser: self }
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