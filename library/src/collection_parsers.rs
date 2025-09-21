// Copyright Rob Gage 2025

mod many;

use many::Many;

use crate::{
    Input,
    ParseResult::{
        self,
        Failure,
        Success
    },
    Parser
};

/// Implementors are parsers that return collections of items
pub trait CollectionParser<E, I, O> where
    Self: Parser<I, Error = E, Output = O>,
    I: Input
{

    /// Requires that this `CollectionParser` parse at least a given number of items
    fn at_least(self, minimum: usize) -> Self;

    /// Requires that this `CollectionParser` parse at most a given number of items
    fn at_most(self, maximum: usize) -> Self;

}

/// Applies a parser repeatedly until it fails, and returns all parsed items as a `Vec<O`
pub const fn many<E, I, O, P>(parser: P)-> impl Parser<I, Error = E, Output = Vec<O>> where
    I: Input,
    P: Parser<I, Output = O, Error = E>,
{
    Many { maximum: None, minimum: 0, parser }
}