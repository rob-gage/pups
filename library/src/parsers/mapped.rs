// Copyright Rob Gage 2025

use crate::{
    Input,
    ParseResult,
    Parser
};


/// A combinator that maps the result type of a parser to another result type
pub struct Mapped<M, P> {
    /// The function used to map the result of the parser
    pub mapper: M,
    /// The parser whose result is mapped
    pub parser: P,
}


impl<EA, EB, I, M, P, OA, OB> Parser<I> for Mapped<M, P> where
    M: Fn(ParseResult<OA, EA>) -> ParseResult<OB, EB>,
    I: Input,
    P: Parser<I, Error = EA, Output = OA>,
{

    type Error = EB;

    type Output = OB;

    fn accept(&self, input: &mut I) -> bool { self.parser.accept(input) }

    fn parse(&self, input: &mut I) -> ParseResult<OB, EB> {
        (self.mapper)(self.parser.parse(input))
    }

}