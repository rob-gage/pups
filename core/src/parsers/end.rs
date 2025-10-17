// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};


/// Requires that there be no input remaining
struct End;

impl<'a, I> Parser<'a, (), (), (), I> for End
where
    I: Input<'a>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<(), (), (), _Mode> {
        if input.peek().is_some() {  Failure (
            _Mode::convert_error(()),
            _Mode::new_message_container()
        )} else { Success (
            _Mode::convert_output(()),
            _Mode::new_message_container()
        )}
    }

    implement_modes!('a, (), (), (), I);

}

/// Matches the end of the provided input
pub const fn end<'a, I>() -> impl Parser<'a, (), (), (), I>
where
    I: Input<'a>,
{ End }