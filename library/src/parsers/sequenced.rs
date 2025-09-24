// Copyright Rob Gage 2025

use crate::{
    Input,
    Mode,
    ParseResult,
    Parser,
};
use std::marker::PhantomData;


/// A combinator that applies one parser, then another, and then returns their outputs as a tuple
pub struct Sequenced<C, Mode1, Mode2, P1, P2> {
    /// The head of the `Sequence`
    pub head: P1,
    /// The tail of the `Sequence`
    pub tail: P2,
    _marker: PhantomData<(C, Mode1, Mode2)>
}

impl<C, E, I, M, O1, O2, P1, P2, Mode1, Mode2> Parser<I>
for Sequenced<C, Mode1, Mode2, P1, P2> where
    I: Input,
    P1: Parser<I, Error = E, Message = M, Output = O1>,
    P2: Parser<I, Error = E, Message = M, Output = O2>,
    Mode1: Mode<E, M, O1, MessageContainer= C>,
    Mode2: Mode<E, M, O2, MessageContainer= C>,
{

    type Error = E;

    type Message = M;

    type Output = (O1, O2);

    fn apply<Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<E, M, (O1, O2), Mode> where
        Mode: Mode<E, M, (O1, O2)>
    {
        let cursor = input.cursor();
        let parsed = self.head.apply::<Mode1>(input);
        let head_result = parsed.to_result();
        let mut messages = parsed.messages;
        match head_result {
            Ok (output_a) => {
                let parsed = self.tail.apply::<Mode2>(input);
                let tail_result = parsed.to_result();
                Mode1::combine_message_containers(&mut messages, parsed.messages);
                match tail_result {
                    Ok (output_b) => ParseResult::<E, M, (O1, O2), Mode>::new_success(
                        (output_a, output_b),
                        messages
                    ),
                    Err (error) => {
                        input.set_cursor(cursor);
                        ParseResult::new_failure(error, messages)
                    }
                }
            }
            Err (error) => ParseResult::new_failure(error, messages)
        }

    }

}