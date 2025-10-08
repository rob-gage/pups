// Copyright Rob Gage 2025

use crate::{
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

impl<E, I, M, O, P> Parser<I> for Optional<P>
where
    I: Input,
    P: Parser<I, Output = O, Error = E, Message = M>,
{

    type Output = Option<O>;

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(&self, input: &mut I) -> ModeResult<Option<O>, E, M, _Mode> {
        match self.0.apply::<_Mode>(input) {
            success @ Success (..) => _Mode::map_output(success, |output| Some (output)),
            Failure (_, messages) => Success (_Mode::convert_output(None), messages)
        }
    }

}