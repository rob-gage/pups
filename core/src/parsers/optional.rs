// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};


/// A combinator that applies a parser optionally
pub struct Optional<P> (pub P);

impl<'a, O, E, M, I, P> Parser<'a, Option<O>, E, M, I> for Optional<P>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<Option<O>, E, M, _Mode> {
        match self.0.apply::<_Mode>(input) {
            success @ Success (..) => _Mode::map_output(success, |output| Some (output)),
            Failure (_, messages) => Success (_Mode::convert_output(None), messages)
        }
    }

    implement_modes!('a, Option<O>, E, M, I);

}