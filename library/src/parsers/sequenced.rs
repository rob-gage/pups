// Copyright Rob Gage 2025

use crate::{
    Input,
    Mode,
    ParseResult::{
        self,
        Success,
        Failure,
    },
    Parser,
};

/// A combinator that applies one parser, then another, and then returns their outputs as a tuple
pub struct Sequenced<P1, P2> {
    /// The head of the `Sequence`
    pub head: P1,
    /// The tail of the `Sequence`
    pub tail: P2,
}

impl<E, I, M, O1, O2, P1, P2> Parser<I> for Sequenced<P1, P2> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E, Message = M>,
    P2: Parser<I, Output = O2, Error = E, Message = M>,
{
    type Output = (O1, O2);

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<(O1, O2), E, M, _Mode> {
        let cursor = input.cursor();
        match self.head.apply::<_Mode>(input) {
            Success (head_output, mut head_messages) => match self.tail.apply::<_Mode>(input) {
                Success (tail_output, tail_messages) => {
                    _Mode::combine_message_containers(&mut head_messages, tail_messages.into());
                    Success (
                        _Mode::convert_output((head_output, tail_output)),
                        head_messages
                    )
                }
            }
            Failure (error, messages) => Failure (_Mode::convert_error(error), messages)
        }
    }

}