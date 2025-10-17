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

pub struct Choice<P1, P2> {
    /// The alternate parser
    pub alternate: P2,
    /// The primary parser
    pub primary: P1,
}

impl<'a, O, E1, E2, M, I, P1, P2> Parser<'a, O, (E1, E2), M, I> for Choice<P1, P2> where
    I: Input<'a>,
    P1: Parser<'a, O, E1, M, I>,
    P2: Parser<'a, O, E2, M, I>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, (E1, E2), M, _Mode> {
        match self.primary.apply::<_Mode>(input) {
            Success (output, messages) => Success (output, messages),
            Failure (primary_error, _) =>
                match self.alternate.apply::<_Mode>(input) {
                    Success (output, alternate_messages) => Success (output, alternate_messages),
                    Failure (alternate_error, alternate_messages) => Failure (
                        _Mode::merge_errors(
                            primary_error,
                            alternate_error,
                            |p, a| (p, a)
                        ),
                        alternate_messages
                    )
                }
        }
    }

    implement_modes!('a, O, (E1, E2), M, I);

}