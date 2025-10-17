// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult,
    Parser
};
use std::marker::PhantomData;


/// A combinator that maps the output type of a parser to another type
pub struct Mapped<OA, F, P> {
    /// The parser whose output is mapped
    pub parser: P,
    /// The function used to map the output of the parser
    pub function: F,
    pub _phantom: PhantomData<OA>,
}


impl<'a, OA, OB, E, M, F, I, P> Parser<'a, OB, E, M, I> for Mapped<OA, F, P>
where
    F: Fn(OA) -> OB + Clone,
    I: Input<'a>,
    P: Parser<'a, OA, E, M, I>,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<OB, E, M, _Mode> {
        _Mode::map_output(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

    implement_modes!('a, OB, E, M, I);

}