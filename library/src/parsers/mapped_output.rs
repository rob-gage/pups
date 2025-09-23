// Copyright Rob Gage 2025

use crate::{
    Input,
    ParseMode,
    ParseResult,
    Parser
};


/// A combinator that maps the output type of a parser to another type by applying a function
pub struct MappedOutput<F, P> {
    /// The function used to map the output of the parser
    pub function: F,
    /// The parser whose result is mapped
    pub parser: P,
}


impl<E, F, I, M, O, _O, P> Parser<I> for MappedOutput<F, P> where
    F: Fn(O) -> _O,
    I: Input,
    P: Parser<I, Error = E, Message = M, Output = O>,
{

    type Error = E;

    type Message = M;

    type Output = _O;

    fn parse<Mode>(&self, input: &mut I) -> Mode::Result<E, M, _O> where
        Mode: ParseMode,
    {
        let parsed: Mode::Result<E, M, O> = self.parser.parse::<Mode>(input);
        let (result, messages): (
            Result<O, E>,
            <Mode::Result<E, M, O> as ParseResult<E, M, O>>::MessageContainer
        ) = parsed.to_result_and_messages();
        let messages: <Mode::Result<E, M, _O> as ParseResult<E, M, _O>>::MessageContainer
            = messages;
        Mode::Result::from_result_and_messages(result.map(self.function), messages)
    }

}