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

/// Parses a number
pub struct Number;

impl<'a, C, I> Parser<'a, I::Slice, (), (), I> for Number
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{
    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<I::Slice , (), (), _Mode> {
        let start: usize = input.store_cursor();
        loop {
            if let Some (character) = input.peek() && character.is_ascii_decimal() {
                input.advance();
            } else { break };
        }
        let end: usize = input.store_cursor();
        if end - start > 0 {
            Success (
                _Mode::convert_output(input.slice(start, end)),
                _Mode::new_message_container()
            )
        } else {
            input.move_cursor(start);
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

    implement_modes!('a, I::Slice, (), (), I);

}

/// Parses a number composed of ASCII decimal digits 0-9
pub const fn number<'a, C, I>(
) -> impl Parser<'a, I::Slice, (), (), I>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ Number }