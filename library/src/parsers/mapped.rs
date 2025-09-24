// Copyright Rob Gage 2025

use crate::{
    Input,
    Parse,
    Mode,
    ParseResult,
    Parser
};


/// A combinator that maps the result type of a parser to another result type
pub struct Mapped<F, P> {
    /// The function used to map the result of the parser
    pub function: F,
    /// The parser whose result is mapped
    pub parser: P,
}


impl<CA, CB, EA, EB, MA, MB, OA, OB, F, I, P, ModeA, ModeB> Parser<I> for Mapped<F, P> where
    F: Fn(ParseResult<EA, MA, OA, ModeA>) -> ParseResult<EB, MB, OB, ModeB>,
    I: Input,
    P: Parser<I, Error = EA, Message = MA, Output = OA>,
{

    type Error = EB;

    type Message = ModeB;

    type Output = OB;

    fn apply<Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<EB, MB, OB, Mode> where
        Mode: Mode<EB, MB, OB>
    {
        (Mode::apply_parser(self.parser, input))
    }

}