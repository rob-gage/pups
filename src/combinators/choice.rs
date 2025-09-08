// Copyright Rob Gage 2025

use crate::{
    Combinators,
    InputStream,
    Parser
};

/// Parser combinator that tries a list of parsers in order and returns the output of the first
/// successful one
pub struct Choice<E, I, O> (pub Vec<Box<dyn Parser<I, Error = E, Output = O>>>);

impl<E, I, O> Choice<E, I, O> where
    I: InputStream
{

    /// Tries parsers in order until one succeeds
    fn or<P>(mut self, next_alternative_parser: P) -> Self where
        P: Parser<I, Error = E, Output = O> + 'static
    {
        self.0.push(Box::new(next_alternative_parser));
        self
    }

}

impl<E, I, O> Parser<I> for Choice<E, I, O> where
    I: InputStream
{

    type Error = E;

    type Output = O;

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        let mut furthest_consumed: usize = input.cursor();
        let mut errors: Vec<E> = Vec::new();
        for parser in &self.0 {
            let cursor: usize = input.cursor(); // save cursor for backtracking
            match parser.parse(input) {
                Ok(output) => return Ok (output), // return first successful result
                Err(new_errors) => {
                    if input.cursor() > furthest_consumed { // return only new errors if this parser
                        errors = new_errors; // consumes more input than all previous parsers
                        furthest_consumed = input.cursor();
                    } else if input.cursor() == furthest_consumed { // add new errors to existing
                        errors.extend(new_errors) // errors this parser consumes just as much
                    }
                    input.set_cursor(cursor); // backtrack to before parser that failed
                }
            }
        }
        Err (errors)
    }

}