// Copyright Rob Gage 2025

use crate::{
    CollectionParser,
    Input,
    ParseResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};
use crate::collection_parsers::many::Many;
use crate::modes::Mode;

/// A combinator that applies a parser repeatedly separated by a separator parser
pub struct Separated<P1, P2> {
    /// The maximum number of items that can be parsed
    pub maximum: Option<usize>,
    /// The minimum number of items that can be parsed
    pub minimum: usize,
    /// The parser that is repeatedly applied
    pub parser: P1,
    /// The separator that is applied between parsed items
    pub separator: P2
}


impl<E, I, O, P1, P2> CollectionParser<E, I, O> for Separated<P1, P2> where
    Self: Parser<I, Output = O, Error = E>,
    I: Input,
{

    fn at_least(self, minimum: usize) -> Self {
        let mut parser: Self = self;
        parser.minimum = minimum;
        parser
    }

    fn at_most(self, maximum: usize) -> Self {
        let mut parser: Self = self;
        parser.maximum = Some(maximum);
        parser
    }

}

impl<E, I, M, O1, O2, P1, P2> Parser<I> for Separated<P1, P2> where
    I: Input,
    P1: Parser<I, Output = O1, Error = E, Message = M>,
    P2: Parser<I, Output = O2, Error = E, Message = M>,
{
    type Output = Vec<O1>;

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<Vec<O1>, E, M, _Mode> {
        let start_cursor: usize = input.cursor();
        let maximum: usize = if let Some (maximum) = self.maximum { maximum } else { usize::MAX };
        let mut outputs: _Mode::OutputForm<Vec<O1>> = _Mode::convert_output(Vec::new());
        let mut message_container: _Mode::MessageContainer<M> = _Mode::new_message_container();
        // parse first output (if it exists)
        let mut output_count: usize = if maximum != 0 {
            match self.parser.apply::<_Mode>(input) {
                Success (output, messages) => {
                    outputs = _Mode::merge_outputs(outputs, output, |mut outputs, output| {
                        outputs.push(output); outputs
                    });
                    message_container = _Mode::merge_message_containers(message_container, messages);
                    1
                }
                Failure (error, messages) => return if self.minimum == 0 {
                    Success (
                        _Mode::convert_output(Vec::new()),
                        _Mode::merge_message_containers(
                            message_container,
                            messages
                        )
                    )
                } else {
                    Failure (error, _Mode::merge_message_containers(
                        message_container,
                        messages
                    ))
                },
            }
        } else { return Success (_Mode::convert_output(Vec::new()), message_container) };
        while output_count < maximum {
            let cursor: usize = input.cursor();
            // parse separator
            match self.separator.apply::<_Mode>(input) {
                Success (_, messages) => {
                    message_container = _Mode::merge_message_containers(
                        message_container,
                        messages
                    );
                    // parse output
                    match self.parser.apply::<_Mode>(input) {
                        Success (output, messages) => {
                            debug_assert!(input.cursor() > cursor);
                            message_container =
                                _Mode::merge_message_containers(message_container, messages);
                            outputs = _Mode::merge_outputs(outputs, output, |mut outputs, output| {
                                outputs.push(output); outputs
                            });
                            output_count += 1;
                        }
                        Failure (error, messages) => {
                            input.set_cursor(start_cursor);
                            return Failure (
                                error,
                                _Mode::merge_message_containers(message_container, messages)
                            )
                        }
                    }
                }
                Failure (error, messages) => if output_count < self.minimum {
                    input.set_cursor(start_cursor);
                    return Failure (
                        error,
                        _Mode::merge_message_containers(message_container, messages)
                    )
                } else { break }
            }
        }
        Success (outputs, message_container)
    }

}

