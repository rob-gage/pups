// Copyright Rob Gage 2025

use crate::{
    Input,
    Mode,
    ModeResult,
    Parser
};


/// A combinator that maps the output type of a parser to another type
pub struct OutputMapper<F, P> {
    /// The parser whose output is mapped
    pub parser: P,
    /// The function used to map the output of the parser
    pub function: F,
}


impl<E, F, I, M, P, OA, OB> Parser<I> for OutputMapper<F, P> where
    F: Fn(OA) -> OB + Clone,
    I: Input,
    P: Parser<I, Output = OA, Error = E, Message = M>,
{
    type Output = OB;

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ModeResult<Self::Output, Self::Error, Self::Message, _Mode> {
        _Mode::map_output(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

}



/// A combinator that maps the error type of a parser to another type
pub struct ErrorMapper<F, P> {
    /// The parser whose error is mapped
    pub parser: P,
    /// The function used to map the error of the parser
    pub function: F,
}


impl<EA, EB, F, I, M, P, O> Parser<I> for ErrorMapper<F, P> where
    F: Fn(EA) -> EB + Clone,
    I: Input,
    P: Parser<I, Output = O, Error = EA, Message = M>,
{
    type Output = O;

    type Error = EB;

    type Message = M;

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ModeResult<Self::Output, Self::Error, Self::Message, _Mode> {
        _Mode::map_error(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

}



/// A combinator that maps the message type of a parser to another type
pub struct MessageMapper<F, P> {
    /// The parser whose messages are mapped
    pub parser: P,
    /// The function used to map the messages of the parser
    pub function: F,
}


impl<E, F, I, MA, MB, P, O> Parser<I> for MessageMapper<F, P> where
    F: Fn(MA) -> MB + Clone,
    I: Input,
    P: Parser<I, Output = O, Error = E, Message = MA>,
{
    type Output = O;

    type Error = E;

    type Message = MB;

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ModeResult<Self::Output, Self::Error, Self::Message, _Mode> {
        _Mode::map_messages(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

}