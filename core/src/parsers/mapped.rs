// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult,
    Parser
};
use std::marker::PhantomData;


/// A combinator that maps the output type of a parser to another type
pub struct OutputMapped<OA, F, P> {
    /// The parser whose output is mapped
    pub parser: P,
    /// The function used to map the output of the parser
    pub function: F,
    pub _phantom: PhantomData<OA>,
}


impl<'a, OA, OB, E, M, F, I, P> Parser<'a, OB, E, M, I> for OutputMapped<OA, F, P>
where
    F: Fn(OA) -> OB + Clone,
    I: Input<'a>,
    P: Parser<'a, OA, E, M, I>,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<OB, E, M, _Mode> {
        _Mode::map_output(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

    implement_modes!('a, OB, E, M, I);

}



/// A combinator that maps the error type of a parser to another type
pub struct ErrorMapped<EA, F, P> {
    /// The parser whose error is mapped
    pub parser: P,
    /// The function used to map the error of the parser
    pub function: F,
    pub _phantom: PhantomData<EA>
}


impl<'a, O, EA, EB, M, F, I, P> Parser<'a, O, EB, M, I> for ErrorMapped<EA, F, P>
where
    F: Fn(EA) -> EB + Clone,
    I: Input<'a>,
    P: Parser<'a, O, EA, M, I>,
{
    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<O, EB, M, _Mode> {
        _Mode::map_error(
            self.parser.apply::<_Mode>(input),
            self.function.clone()
        )
    }

    implement_modes!('a, O, EB, M, I);

}



/// A combinator that maps the message type of a parser to another type
pub struct MessagesMapped<MA, F, P> {
    /// The parser whose messages are mapped
    pub parser: P,
    /// The function used to map the messages of the parser
    pub function: F,
    pub _phantom: PhantomData<MA>,
}


impl<'a, O, E, MA, MB, F, I, P> Parser<'a, O, E, MB, I> for MessagesMapped<MA, F, P>
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