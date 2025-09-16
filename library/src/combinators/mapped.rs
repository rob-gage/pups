// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
};

/// See `mapped`
struct Mapped<M, P> {
    /// The function used to map the result of the parser
    mapper: M,
    /// The parser whose result is mapped
    parser: P,
}

impl<EA, EB, I, M, P, OA, OB> Parser<I> for Mapped<M, P> where
    M: Fn(Result<OA, Vec<EA>>) -> Result<OB, Vec<EB>>,
    I: Input,
    P: Parser<I, Error = EA, Output = OA>,
{

    type Error = EB;

    type Output = OB;

    fn accept(&self, input: &mut I) -> bool { self.parser.accept(input) }

    fn parse(&self, input: &mut I) -> Result<OB, Vec<EB>> {
        (self.mapper)(self.parser.parse(input))
    }

}

/// Maps the result of one parser into a new type using a mapper function
pub const fn mapped<EA, EB, I, M, OA, OB, P>(
    parser: P,
    mapper: M
) -> impl Parser<I, Error = EB, Output = OB> where
    I: Input,
    M: Fn(Result<OA, Vec<EA>>) -> Result<OB, Vec<EB>>,
    P: Parser<I, Error = EA, Output = OA>,
{ Mapped { mapper, parser } }
