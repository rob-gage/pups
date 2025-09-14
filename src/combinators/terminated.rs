// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
};

/// Parser combinator that returns the output of a parser followed by another parser
struct Terminated<PI, PM> {
    /// The parser whose output is ignored after the main parser
    ignored_parser: PI,
    /// The parser whose output is returned
    main_parser: PM,
}

impl<E, I, OI, OM, PI, PM> Parser<I> for Terminated<PI, PM> where
    I: Input,
    PI: Parser<I, Output = OI, Error = E>,
    PM: Parser<I, Output = OM, Error = E>,
{

    type Error = E;

    type Output = OM;

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        let cursor: usize = input.cursor();
        match self.main_parser.parse(input) {
            Err (errors) => {
                input.set_cursor(cursor);
                Err (errors)
            }
            Ok (output) => match self.ignored_parser.parse(input) {
                Err (errors) => {
                    input.set_cursor(cursor);
                    Err (errors)
                }
                _ => Ok (output)
            }
        }
    }

}

/// Parser combinator that returns the output of a parser followed by another parser
pub const fn terminated<E, I, OI, OM, PI, PM>(
    main_parser: PM,
    ignored_parser: PI
) -> impl Parser<I, Error = E, Output = OM> where
    I: Input,
    PI: Parser<I, Output = OI, Error = E>,
    PM: Parser<I, Output = OM, Error = E>,
{
    Terminated {
        ignored_parser,
        main_parser,
    }
}
