// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput
};
use pups_core::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser
};

struct Newline;

impl<'a, C, I> Parser<'a, (), (), (), I> for Newline
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<(), (), (), _Mode> {
        if let Some (character) = input.peek() && character.is_newline() {
            input.advance();
            Success (_Mode::convert_output(()), _Mode::new_message_container())
        } else {
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

    implement_modes!('a, (), (), (), I);

}

/// Parses a single newline character
pub const fn newline<'a, C, I>() -> impl Parser<'a, (), (), (), I>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ Newline }