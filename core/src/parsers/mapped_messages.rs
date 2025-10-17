// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult,
    Parser
};
use std::marker::PhantomData;

/// A combinator that maps the message type of a parser to another type
struct MappedMessages<MA, F, P> {
    /// The parser whose messages are mapped
    parser: P,
    /// The function used to map the messages of the parser
    function: F,
    _phantom: PhantomData<MA>,
}

impl<'a, O, E, MA, MB, F, I, P> Parser<'a, O, E, MB, I> for MappedMessages<MA, F, P>
where
    F: Fn(MA) -> MB + Clone,
    I: Input<'a>,
    P: Parser<'a, O, E, MA, I>,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<O, E, MB, _Mode> {
        _Mode::map_messages(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

    implement_modes!('a, O, E, MB, I);

}

/// Maps a parser's output to another type using a function
pub const fn mapped_messages<'a, O, E, MA, MB, I>(
    parser: impl Parser<'a, O, E, MA, I>,
    function: impl Fn(MA) -> MB + Clone
) -> impl Parser<'a, O, E, MB, I>
where
    I: Input<'a>,
{ MappedMessages { parser, function, _phantom: PhantomData } }