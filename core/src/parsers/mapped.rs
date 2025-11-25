// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult,
    Parser
};
use std::marker::PhantomData;

pub struct Mapped<OA, F, P> {
    /// The parser whose output is mapped
    parser: P,
    /// The function used to map the output of the parser
    function: F,
    _phantom: PhantomData<OA>,
}

impl<'a, OA, OB, E, M, F, I, P> Parser<'a, OB, E, M, I> for Mapped<OA, F, P>
where
    F: Fn(OA) -> OB,
    I: Input<'a>,
    P: Parser<'a, OA, E, M, I>,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<OB, E, M, _Mode> {
        _Mode::map_output(
            self.parser.apply::<_Mode>(input),
            &self.function
        )
    }

    implement_modes!('a, OB, E, M, I);

}

/// Maps a parser's output to another type using a function
pub const fn mapped<'a, OA, OB, E, M, I>(
    parser: impl Parser<'a, OA, E, M, I>,
    function: impl Fn(OA) -> OB
) -> impl Parser<'a, OB, E, M, I>
where
    I: Input<'a>,
{ Mapped { parser, function, _phantom: PhantomData } }