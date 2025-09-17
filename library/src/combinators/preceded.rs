// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
};


/// See `preceded`
struct Preceded<P1, P2> {
    /// The parser whose output is returned
    parser: P2,
    /// The prefix whose output is ignored
    prefix: P1,
}


impl<E, I, O1, O2, P1, P2> Parser<I> for Preceded<P1, P2> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
{

    type Error = E;

    type Output = O2;

    fn accept(&self, input: &mut I) -> bool {
        let cursor: usize = input.cursor();
        if !self.prefix.accept(input) {
            input.set_cursor(cursor); return false;
        }
        if !self.parser.accept(input) {
            input.set_cursor(cursor); false
        } else { true }
    }

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        let cursor: usize = input.cursor();
        match self.prefix.parse(input) {
            Err (errors) => {
                input.set_cursor(cursor);
                Err (errors)
            }
            _ => match self.parser.parse(input) {
                Err (errors) => {
                    input.set_cursor(cursor);
                    Err (errors)
                }
                ok => ok
            }
        }
    }

}


/// Parses input after a prefix
pub const fn preceded<E, I, O1, O2, P1, P2>(
    prefix: P1,
    parser: P2
) -> impl Parser<I, Error = E, Output = O2> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
{ Preceded { parser, prefix} }
