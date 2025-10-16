// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success
    },
    Parser
};
use std::marker::PhantomData;

/// A combinator that applies a parser in multiple iterations
pub struct Iterated<O2, P1, P2> {
    /// The maximum number of parser iterations that this combinator applies
    pub maximum: Option<usize>,
    /// The minimum number of parser iterations that this combinator applies
    pub minimum: usize,
    /// The parser that is applied in multiple iterations
    pub parser: P1,
    /// The parser that is applied in between iterations
    pub separator: P2,
    pub _phantom: PhantomData<O2>,
}

impl<O2, P1, P2> Iterated<O2, P1, P2> {

    /// Requires that this iterated parser combinator parse at least a given number of outputs
    pub const fn at_least(mut self, minimum: usize) -> Self {
        self.minimum = minimum;
        self
    }

    /// Requires that this iterated parser combinator parse no more than a given number of outputs
    pub const fn at_most(mut self, maximum: usize) -> Self {
        self.maximum = Some(maximum);
        self
    }

}

impl<'a, O1, O2, E, M, I, P1, P2> Parser<'a, Vec<O1>, E, M, I> for Iterated<O2, P1, P2>
where
    I: Input<'a>,
    P1: Parser<'a, O1, E, M, I>,
    P2: Parser<'a, O2, E, M, I>,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<Vec<O1>, E, M, _Mode> {
        let start_cursor: usize = input.save_cursor();
        let maximum: usize = if let Some (maximum) = self.maximum { maximum } else { usize::MAX };
        let mut outputs: _Mode::OutputForm<Vec<O1>> = _Mode::convert_output(Vec::new());
        let mut output_count: usize = 0;
        let mut message_container: _Mode::MessageContainer<M> = _Mode::new_message_container();
        while output_count < maximum {
            // parse separator
            match self.separator.apply::<_Mode>(input) {
                Success (_, messages) => {
                    message_container =
                        _Mode::merge_message_containers(message_container, messages);
                    // parse output
                    let cursor: usize = input.save_cursor();
                    match self.parser.apply::<_Mode>(input) {
                        Success (output, messages) => {
                            debug_assert!(input.save_cursor() > cursor);
                            message_container =
                                _Mode::merge_message_containers(message_container, messages);
                            outputs = _Mode::merge_outputs(outputs, output, |mut outputs, output| {
                                outputs.push(output); outputs
                            });
                            output_count += 1;
                        }
                        Failure (error, messages) => if output_count < self.minimum {
                            input.restore_cursor(start_cursor);
                            return Failure (
                                error,
                                _Mode::merge_message_containers(message_container, messages)
                            )
                        } else { break }
                    }
                }
                Failure (error, messages) => if output_count < self.minimum {
                    // fail if more iterations were required
                    input.restore_cursor(start_cursor);
                    return Failure (
                        error,
                        _Mode::merge_message_containers(message_container, messages)
                    )
                } else { break }
            }
        }
        Success (outputs, message_container)
    }

    implement_modes!('a, Vec<O1>, E, M, I);

}