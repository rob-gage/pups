// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
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