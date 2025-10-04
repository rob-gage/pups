// Copyright Rob Gage 2025

use crate::{
    Input,
    Mode,
    ParseResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};


/// A combinator that applies a primary parser, and then an alternate, returning the output from
/// the first of the two to succeed, or the errors from both as a tuple
pub struct Choice<P1, P2> {
    /// The alternate parser
    pub alternate: P2,
    /// The primary parser
    pub primary: P1,
}

impl<E1, E2, I, M, O, P1, P2> Parser<I> for Choice<P1, P2> where
    I: Input,
    P1: Parser<I, Output = O, Error = E1, Message = M>,
    P2: Parser<I, Output = O, Error = E2, Message = M>,
{

    type Output = O;

    type Error = (E1, E2);

    type Message = M;

    fn apply<_Mode: Mode>(&self, input: &mut I) -> ParseResult<O, (E1, E2), M, _Mode> {
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

}