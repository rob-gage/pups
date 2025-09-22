// Copyright Rob Gage 2025

use crate::{
    Input,
    ParseMode,
    ParseResult,
    Parser
};
use std::marker::PhantomData;


/// A combinator that maps the result type of a parser to another result type
pub struct Mapped<EA, EB, F, MA, MB, OA, OB, P, RA, RB> {
    /// The function used to map the result of the parser
    pub function: F,
    /// The parser whose result is mapped
    pub parser: P,
    _marker: PhantomData<(EA, EB, MA, MB, OA, OB, RA, RB)>,
}


impl<EA, EB, F, I, P, MA, MB, OA, OB, RA, RB> Parser<I>
for Mapped<EA, EB, F, MA, MB, OA, OB, P, RA, RB> where
    F: Fn(RA) -> RB,
    I: Input,
    P: Parser<I, Error = EA, Message = MA, Output = OA>,
    RA: ParseResult<EA, MA, OA>,
    RB: ParseResult<EB, MB, OB>,
{

    type Error = EB;

    type Message = MB;

    type Output = OB;

    fn parse<Mode>(&self, input: &mut I) -> Mode::Result<EB, MB, OB> where
        Mode: ParseMode
    {
        let result: RA = self.parser.parse(input);
        (self.function)(result)
    }

}