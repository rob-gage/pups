// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult,
    Parser
};
use std::marker::PhantomData;

/// A combinator that maps the error type of a parser to another type
pub struct MappedError<EA, F, P> {
    /// The parser whose error is mapped
    pub parser: P,
    /// The function used to map the error of the parser
    pub function: F,
    pub _phantom: PhantomData<EA>
}


impl<'a, O, EA, EB, M, F, I, P> Parser<'a, O, EB, M, I> for MappedError<EA, F, P>
where
    F: Fn(EA) -> EB + Clone,
    I: Input<'a>,
    P: Parser<'a, O, EA, M, I>,
{
    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<O, EB, M, _Mode> {
        _Mode::map_error(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

    implement_modes!('a, O, EB, M, I);

}