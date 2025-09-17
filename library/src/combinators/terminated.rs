// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
};


/// See `terminated`
struct Terminated<P2, P1> {
    /// The parser whose output is returned
    parser: P1,
    /// The terminator whose output is ignored
    terminator: P2,
}


impl<E, I, O2, O1, P2, P1> Parser<I> for Terminated<P2, P1> where
    I: Input,
    P2: Parser<I, Output = O2, Error = E>,
    P1: Parser<I, Output = O1, Error = E>,
{
    type Error = E;

    type Output = O1;

    fn accept(&self, input: &mut I) -> bool {
        let cursor: usize = input.cursor();
        if !self.parser.accept(input) {
            input.set_cursor(cursor); return false;
        }
        if !self.terminator.accept(input) {
            input.set_cursor(cursor); false
        } else { true }
    }

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        let cursor: usize = input.cursor();
        match self.parser.parse(input) {
            Err(errors) => {
                input.set_cursor(cursor);
                Err(errors)
            }
            Ok(output) => match self.terminator.parse(input) {
                Err(errors) => {
                    input.set_cursor(cursor);
                    Err(errors)
                }
                _ => Ok(output)
            }
        }
    }
}


/// Parses input before a terminator
pub const fn terminated<E, I, O2, O1, P2, P1>(
    parser: P1,
    terminator: P2
) -> impl Parser<I, Error = E, Output = O1> where
    I: Input,
    P2: Parser<I, Output = O2, Error = E>,
    P1: Parser<I, Output = O1, Error = E>,
{ Terminated { parser, terminator } }
