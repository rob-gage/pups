// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    mapped,
    ModeResult::{
        self,
        Success,
        Failure,
    },
    Parser,
};

pub struct Sequenced<P1, P2> {
    /// The head of the `Sequence`
    head: P1,
    /// The tail of the `Sequence`
    tail: P2,
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
        let cursor = input.store_cursor();
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
                    input.move_cursor(cursor);
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

/// Applies a parser preceded by an ignored prefix parser, and followed by an ignored terminator
/// parser
pub fn delimited<'a, O1, O2, O3, E, M, I, P1, P2, P3>(
    prefix: P1,
    parser: P2,
    terminator: P3,
) -> impl Parser<'a, O2, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
    P3: Parser<'a, O3, E, M, I>,
{ preceded(prefix, terminated(parser, terminator)) }

/// Applies a parser after an ignored prefix parser
pub const fn preceded<'a, O1, O2, E,  M, I, P1, P2>(
    prefix: P1,
    parser: P2
) -> impl Parser<'a, O2, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{ mapped(sequenced(prefix, parser), |(_, output)| output) }

/// Applies a parser followed by another parser, and returns the outputs as a tuple
pub const fn sequenced<'a, O1, O2, E, M, I, P1, P2>(
    first: P1,
    second: P2
) -> impl Parser<'a, (O1, O2), E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{ Sequenced { head: first, tail: second } }

/// Applies a parser followed by an ignored terminator parser
pub const fn terminated<'a, O1, O2, E, M, I, P1, P2>(
    parser: P1,
    terminator: P2
) -> impl Parser<'a, O1, E, M, I>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{ mapped(sequenced(parser, terminator), |(output, _)| output) }