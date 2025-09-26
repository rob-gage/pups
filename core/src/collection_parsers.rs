// Copyright Rob Gage 2025

mod many;
mod separated;

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


// /// Applies a parser repeatedly until it fails
// pub const fn many<E, I, O, P>(parser: P)-> impl Parser<I, Error = E, Output = Vec<O>> where
//     I: Input,
//     P: Parser<I, Output = O, Error = E>,
// { Many { maximum: None, minimum: 0, parser } }
//
//
// /// Applies a parser repeatedly until it fails while applying a separator parser between items
// pub const fn separated<E, I, O1, O2, P1, P2>(
//     parser: P1,
//     separator: P2
// )-> impl Parser<I, Error = E, Output = Vec<O1>> where
//     I: Input,
//     P1: Parser<I, Output = O1, Error = E>,
//     P2: Parser<I, Output = O2, Error = E>,
// { Separated { maximum: None, minimum: 0, parser, separator} }