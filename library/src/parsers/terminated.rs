// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
};


/// A combinator that returns the result of a parser followed by another
/// parser, with the result of this second parser being ignored
pub struct Terminated<P2, P1> {
    /// The parser whose output is returned
    pub parser: P1,
    /// The terminator parser whose output is ignored
    pub terminator: P2,
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
