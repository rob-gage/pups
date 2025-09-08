// Copyright Rob Gage 2025

use crate::{
    InputStream,
    Parser,
};

/// Parser combinator that maps a parser's accumulated errors to a new type using a function
pub struct MapError<F, P> {
    /// The function used to map the errors
    pub function: F,
    /// The parser whose errors are mapped
    pub parser: P,
}

impl<EA, EB, F, I, O, P> Parser<I> for MapError<F, P>
where
    F: Fn(EA) -> EB,
    I: InputStream,
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