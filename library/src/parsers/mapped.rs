// Copyright Rob Gage 2025

use crate::{
    Input,
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


impl<EA, EB, F, I, MA, MB, P, OA, OB> Parser<I> for Mapped<F, P> where
    F: Fn(ParseResult<OA, EA>) -> ParseResult<OB, EB>,
    I: Input,
    P: Parser<I, Error = EA, Output = OA>,
{

    type Error = EB;

    type Output = OB;


}