// Copyright Rob Gage 2025

use crate::{
    Input,
    ParseResult,
    Parser,
};

/// A combinator that applies one parser, and then another if the first fails, and returns
/// the output from the one that succeeds or the errors from both if they fail
pub struct Choice<P1, P2> {
    /// The alternative parser
    pub alternative: P2,
    /// The primary parser
    pub primary: P1,
}

impl<E, I, O, P1, P2> Parser<I> for Choice<P1, P2> where
    I: Input,
    P1: Parser<I, Error = E, Output = O>,
    P2: Parser<I, Error = E, Output = O>,
{

    type Error = E;

    type Output = O;

    fn accept(&self, input: &mut I) -> bool {
        self.primary.accept(input) || self.alternative.accept(input)
    }

    fn parse(&self, input: &mut I) -> ParseResult<O, E> {
        self.primary.parse(input)).or_else(|mut primary_errors| {
            self.alternative.parse(input).map_err(|alternative_errors| {
                primary_errors.extend(alternative_errors);
                ParseResult::from_errors(primary_errors)
            })
        })
    }

}