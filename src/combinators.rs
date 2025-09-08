// Copyright Rob Gage 2025

mod map;
mod map_error;

use crate::{
    InputStream,
    Parser,
};
use map::Map;
use map_error::MapError;

/// Helper methods implemented for all parsers that allows easy construction of combinators
pub trait Combinators<E, I, O>
where
    Self: Parser<I, Error = E, Output = O> + Sized,
    I: InputStream
{

    /// Maps a parser's output to another type using a function
    fn map<F>(self, f: F) -> Map<F, Self> {
        Map { function: f, parser: self }
    }

    /// Maps each of a parser's accumulated errors to a new type using a function
    fn map_error<F>(self, f: F) -> MapError<F, Self> {
        MapError { function: f, parser: self }
    }

}

impl<E, I, O, T> Combinators<E, I, O> for T
where
    I: InputStream,
    T: Parser<I, Error = E, Output = O>
{ }