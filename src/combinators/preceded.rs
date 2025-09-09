// Copyright Rob Gage 2025

use crate::{
    InputStream,
    Parser,
};

/// Parser combinator that returns the output of a parser preceded by another parser
struct Preceded<PI, PM> {
    /// The parser whose output is ignored before the main parser
    ignored_parser: PI,
    /// The parser whose output is returned
    main_parser: PM,
}

impl<E, I, OI, OM, PI, PM> Parser<I> for Preceded<PI, PM> where
    I: InputStream,
    PI: Parser<I, Output = OI, Error = E>,
    PM: Parser<I, Output = OM, Error = E>,
{

    type Error = E;

    type Output = OM;

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        let cursor: usize = input.cursor();
        match self.ignored_parser.parse(input) {
            Err (errors) => {
                input.set_cursor(cursor);
                Err (errors)
            }
            _ => match self.main_parser.parse(input) {
                Err (errors) => {
                    input.set_cursor(cursor);
                    Err (errors)
                }
                ok => ok
            }
        }
    }

}

/// Parser combinator that returns the output of a parser preceded by another parser
pub const fn preceded<PI, PM>(ignored_parser: PI, main_parser: PM) -> Preceded<PI, PM> {
    Preceded {
        ignored_parser,
        main_parser,
    }
}
