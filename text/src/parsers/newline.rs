// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput
};
use pups_core::{
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser
};

/// Parses a lexical token
pub struct Newline;

impl<'a, C, I> Parser<'a, I> for Newline
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{
    type Output = ();

    type Error = ();

    type Message = ();

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<(), (), (), _Mode> {
        let cursor: usize = input.save();
        if let Some (character) = input.next() && character.is_newline() {
            Success (_Mode::convert_output(()), _Mode::new_message_container())
        } else {
            input.restore(cursor);
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

}