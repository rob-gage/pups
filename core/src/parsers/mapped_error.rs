// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult,
    Parser
};
use std::marker::PhantomData;

struct MappedError<EA, F, P> {
    /// The parser whose error is mapped
    parser: P,
    /// The function used to map the error of the parser
    function: F,
    _phantom: PhantomData<EA>
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

/// Maps a parser's output to another type using a function
pub const fn mapped_error<'a, O, EA, EB, M, I>(
    parser: impl Parser<'a, O, EA, M, I>,
    function: impl Fn(EA) -> EB + Clone
) -> impl Parser<'a, O, EB, M, I>
where
    I: Input<'a>,
{ MappedError { parser, function, _phantom: PhantomData } }