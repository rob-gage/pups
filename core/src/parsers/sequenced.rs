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
    Parser,
};

/// A combinator that applies one parser, then another, and then returns their outputs as a tuple
pub struct Sequenced<P1, P2> {
    /// The head of the `Sequence`
    pub head: P1,
    /// The tail of the `Sequence`
    pub tail: P2,
}

impl<'a, O1, O2, E, M, I, P1, P2> Parser<'a, (O1, O2), E, M, I> for Sequenced<P1, P2>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<(O1, O2), E, M, _Mode> {
        let cursor = input.save_cursor();
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
                    input.restore_cursor(cursor);
                    Failure (
                        tail_error,
                        _Mode::merge_message_containers(head_messages, tail_messages)
                    )
                }
            }
            Failure (head_error, head_messages) => Failure (head_error, head_messages),
        }
    }

    implement_modes!('a, (O1, O2), E, M, I);

}