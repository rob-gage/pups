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
use crate::modes::Mode;

/// A combinator that applies a parser repeatedly
pub struct Many<P> {
    /// The maximum number of items that can be parsed
    pub maximum: Option<usize>,
    /// The minimum number of items that can be parsed
    pub minimum: usize,
    /// The parser that is repeatedly applied
    pub parser: P,
}


impl<E, I, O, P> CollectionParser<E, I, O> for Many<P> where
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


impl<E, I, M, O, P> Parser<I> for Many<P> where
    I: Input,
    P: Parser<I, Output = O, Error = E, Message = M>,
{
    type Output = Vec<O>;

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<Vec<O>, E, M, _Mode> {
        let start_cursor: usize = input.cursor();
        let maximum: usize = if let Some (maximum) = self.maximum { maximum } else { usize::MAX };
        let mut outputs: _Mode::OutputForm<Vec<O>> = _Mode::convert_output(Vec::new());
        let mut output_count: usize = 0;
        let mut message_container: _Mode::MessageContainer<M> = _Mode::new_message_container();
        while output_count < maximum {
            let cursor: usize = input.cursor();
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

