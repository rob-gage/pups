// Copyright Rob Gage 2025

use crate::{
    Combinators,
    Input,
    Parser,
};

/// A sequence of parsers that output optional values that are returned as a `Vec<_>`
pub struct Sequence<P1, P2> {
    /// The head of the `Sequence`
    pub head: P1,
    /// The tail of the `Sequence`
    pub tail: P2,
}

impl<E, I, O1, O2, P1, P2> Parser<I> for Sequence<P1, P2> where
    I: Input,
    P1: Parser<I, Error = E, Output = O1>,
    P2: Parser<I, Error = E, Output = O2>,
{

    type Error = E;

    type Output = (O1, O2);

    fn accept(&self, input: &mut I) -> bool {
        let cursor: usize = input.cursor();
        if !self.head.accept(input) {
            input.set_cursor(cursor);
            return false;
        }
        if !self.tail.accept(input) {
            input.set_cursor(cursor);
            false
        } else { true }
    }

    fn parse(&self, input: &mut I) -> Result<(O1, O2), Vec<E>> {
        let cursor: usize = input.cursor();
        let head: O1 = match self.head.parse(input) {
            Ok (output) => output,
            Err (errors) => {
                input.set_cursor(cursor);
                return Err (errors)
            },
        };
        let tail: O2 = match self.tail.parse(input) {
            Ok (output) => output,
            Err (errors) => {
                input.set_cursor(cursor);
                return Err (errors)
            },
        };
        Ok ((head, tail))
    }

}