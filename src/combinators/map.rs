// Copyright Rob Gage 2025

use crate::{
    InputStream,
    Parser,
};

/// Parser combinator that maps a parser's output to another type using a function
pub struct Map<F, P> {
    /// The function used to map the output
    pub function: F,
    /// The parser whose output is mapped
    pub parser: P,
}

impl<E, F, I, OA, OB, P> Parser<I> for Map<F, P>
where
    F: Fn(OA) -> OB,
    I: InputStream,
    P: Parser<I, Output =OA, Error = E>
{

    type Error = E;

    type Output = OB;

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        self.parser
            .parse(input)
            .map(|result| (self.function)(result))
    }

}