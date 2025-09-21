// Copyright Rob Gage 2025

use crate::{
    Input,
    ParseResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};

/// A combinator that applies one parser, then another, and then returns their outputs as a tuple
pub struct Sequenced<P1, P2> {
    /// The head of the `Sequence`
    pub head: P1,
    /// The tail of the `Sequence`
    pub tail: P2,
}

impl<E, I, O1, O2, P1, P2> Parser<I> for Sequenced<P1, P2> where
    I: Input,
    P1: Parser<I, Error = E, Output = O1>,
    P2: Parser<I, Error = E, Output = O2>,
{

    type Error = E;

    type Output = (O1, O2);

    fn parse(&self, input: &mut I) -> ParseResult<Self::Output, Self::Error> {
        let cursor: usize = input.cursor();
        match self.head.parse(input) {
            Failure (head_errors) => Failure (head_errors),
            Success (head_output, mut head_errors) => match self.tail.parse(input) {
                Failure (tail_errors) => {
                    input.set_cursor(cursor);
                    head_errors.extend(tail_errors);
                    Failure (head_errors)
                }
                Success (tail_output, tail_errors) => {
                    head_errors.extend(tail_errors);
                    Success ((head_output, tail_output), head_errors)
                }
            }
        }
    }

}