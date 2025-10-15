// Copyright Rob Gage 2025

use crate::{
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};


/// A parser that consumes input until finding the first successful match of a child parser, if it
/// exists
pub struct FirstMatch<P> (pub P);

impl<'a, O, E, M, I, P> Parser<'a, Option<O>, E, M, I> for FirstMatch<P>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<Option<O>, E, M, _Mode> {
        while let Some(_) = input.peek() {
            match self.0.apply::<_Mode>(input) {
                Failure (_, _) => input.advance(),
                success => return _Mode::map_output(success, |output| Some (output)),
            }
        }
        Success (_Mode::convert_output(None), _Mode::new_message_container())
    }

}