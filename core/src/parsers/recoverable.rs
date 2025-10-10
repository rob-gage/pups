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

/// A parser combinator that applies a parser, and tries a fallback parser if the first fails
pub struct Recoverable<P1, P2> {
    /// The fallback parser
    pub fallback: P2,
    /// The parser that is applied first
    pub parser: P1,
}

impl<'a, E, I, M, O, P1, P2> Parser<'a, I> for Recoverable<P1, P2> where
    I: Input<'a>,
    P1: Parser<'a, I, Output = O, Error = E, Message = M>,
    P2: Parser<'a, I, Output = O, Error = E, Message = M>,
{

    type Output = O;

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(&self, input: &mut I) -> ModeResult<O, E, M, _Mode> {
        match self.parser.apply::<_Mode>(input) {
            Failure (_, primary_messages) => match self.fallback.apply::<_Mode>(input) {
                    Failure (alternate_error, alternate_messages) => Failure (
                        alternate_error,
                        _Mode::merge_message_containers(primary_messages, alternate_messages),
                    ),
                    success => success
                }
            success => success
        }
    }

}