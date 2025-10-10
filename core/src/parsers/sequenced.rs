// Copyright Rob Gage 2025

use crate::{
    Input,
    Mode,
    ModeResult::{
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

impl<'a, E, I, M, O1, O2, P1, P2> Parser<'a, I> for Sequenced<P1, P2> where
    I: Input<'a>,
    P1: Parser<'a, I, Output = O1, Error = E, Message = M>,
    P2: Parser<'a, I, Output = O2, Error = E, Message = M>,
{
    type Output = (O1, O2);

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ModeResult<(O1, O2), E, M, _Mode> {
        let cursor = input.cursor();
        match self.head.apply::<_Mode>(input) {
            Success (head_output, head_messages) => match self.tail.apply::<_Mode>(input) {
                Success (tail_output, tail_messages) => {
                    Success (
                        _Mode::merge_outputs::<O1, O2, (O1, O2)>(
                            head_output,
                            tail_output,
                            |h, t| (h, t)),
                        _Mode::merge_message_containers(head_messages, tail_messages),
                    )
                }
                Failure (tail_error, tail_messages) => {
                    input.set_cursor(cursor);
                    Failure (
                        tail_error,
                        _Mode::merge_message_containers(head_messages, tail_messages)
                    )
                }
            }
            Failure (head_error, head_messages) => Failure (head_error, head_messages),
        }
    }

}