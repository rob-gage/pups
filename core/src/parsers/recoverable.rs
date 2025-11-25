// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
    },
    Parser,
};

pub struct Recoverable<P1, P2> {
    /// The fallback parser
    fallback: P2,
    /// The parser that is applied first
    parser: P1,
}

impl<'a, O, E, M, I, P1, P2> Parser<'a, O, E, M, I> for Recoverable<P1, P2>
where
    I: Input<'a>,
    P1: Parser<'a, O, E, M, I>,
    P2: Parser<'a, O, E, M, I>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
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

    implement_modes!('a, O, E, M, I);

}

/// Applies a parser, but uses another one to recover if the first fails, keeping messages from both
pub const fn recoverable<'a, O, E, M, I, P1, P2>(
    parser: P1,
    fallback: P2
) -> impl Parser<'a, O, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O, E, M, I>,
    P2: Parser<'a, O, E, M, I>,
{ Recoverable { fallback, parser } }