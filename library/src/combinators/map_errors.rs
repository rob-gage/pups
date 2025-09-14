// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
};

/// Parser combinator that maps a parser's accumulated errors to a new type using a function
pub struct MapErrors<F, P> {
    /// The function used to map the errors
    pub function: F,
    /// The parser whose errors are mapped
    pub parser: P,
}

impl<EA, EB, F, I, O, P> Parser<I> for MapErrors<F, P>
where
    F: Fn(EA) -> EB,
    I: Input,
    P: Parser<I, Output = O, Error = EA>
{

    type Error = EB;

    type Output = O;

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        self.parser
            .parse(input)
            .map_err(|errors| errors.into_iter()
                .map(|error| (self.function)(error))
                .collect()
            )
    }

}