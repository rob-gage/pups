// Copyright Rob Gage 2025

use crate::{
    Combinators,
    Input,
    Parser,
};

/// Parser combinator tries one child parser, and then the next until one succeeds
pub struct Sequence<E, I, O> (pub Vec<(Box<dyn Parser<I, Error = E, Output = O>>, bool)>);

impl<E, I, O> Sequence<E, I, O> where
    I: Input
{

    /// Applies another parser in sequence
    pub fn then<P>(self, next_parser: P) -> Sequence<E, I, O> where
        P: Parser<I, Error = E, Output = O> + 'static
    {
        let mut parsers: Vec<_> = self.0;
        parsers.push((Box::new(next_parser), true));
        Sequence(parsers)
    }

    /// Applies another parser in sequence, but ignores its result
    pub fn then_ignore<P>(self, next_parser: P) -> Sequence<E, I, O> where
        P: Parser<I, Error = E, Output = O> + 'static
    {
        let mut parsers: Vec<_> = self.0;
        parsers.push((Box::new(next_parser), false));
        Sequence(parsers)
    }

}

impl<E, I, O> Parser<I> for Sequence<E, I, O> where
    I: Input
{

    type Error = E;

    type Output = Vec<O>;

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        let cursor: usize = input.cursor(); // save cursor for backtracking
        let mut outputs: Vec<O> = Vec::new(); // accumulate outputs
        for parser in &self.0 {
            match parser.0.parse(input) {
                Ok (output) => if parser.1 { outputs.push(output) }, // push output conditionally
                Err (errors) => {
                    input.set_cursor(cursor); // backtrack
                    return Err (errors)
                }
            }
        }
        Ok(outputs)
    }

}