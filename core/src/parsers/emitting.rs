// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Success,
        Failure,
    },
    Parser
};
use std::marker::PhantomData;

pub struct Emitting<OA, OB, P> {
    /// The new output
    output: OB,
    /// The parser that has its output replaced
    parser: P,
    _phantom: PhantomData<OA>,
}

impl<'a, OA, OB, E, M, I, P> Parser<'a, OB, E, M, I> for Emitting<OA, OB, P>
where
    I: Input<'a>,
    OB: Clone,
    P: Parser<'a, OA, E, M, I>,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<OB, E, M, _Mode> {
        match self.parser.apply::<_Mode>(input) {
            Success (_, messages) => Success (_Mode::convert_output(self.output.clone()), messages),
            Failure (error, messages) => Failure (error, messages),
        }
    }

    implement_modes!('a, OB, E, M, I);

}

/// Replaces a parser's output
pub const fn emitting<'a, OA, OB, E, M, I>(
    parser: impl Parser<'a, OA, E, M, I>,
    output: OB,
) -> impl Parser<'a, OB, E, M, I>
where
    I: Input<'a>,
    OB: Clone,
{ Emitting { output, parser, _phantom: PhantomData } }