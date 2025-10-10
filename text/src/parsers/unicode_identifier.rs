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

/// Parses a unicode identifier
pub struct UnicodeIdentifier;

impl<'a, C, I> Parser<'a, I> for UnicodeIdentifier
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{
    type Output = String;

    type Error = ();

    type Message = ();

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ModeResult<String, (), (), _Mode> {
        let cursor: usize = input.cursor();
        if let Some (character) = input.next() && character.is_unicode_identifier_start() {
            let mut identifier: _Mode::OutputForm<String> = _Mode::convert_output({
                let mut identifier: String = String::new();
                character.write(&mut identifier);
                identifier
            });
            loop {
                let cursor: usize = input.cursor();
                let Some (character) = input.next() else { break; };
                if !character.is_unicode_identifier_continuation() {
                    input.set_cursor(cursor);
                    break;
                };
                identifier = _Mode::merge_outputs(
                    identifier,
                    _Mode::convert_output(character),
                    |mut i, c: C| { c.write(&mut i); i }
                );
            }
            Success (identifier, _Mode::new_message_container())
        } else {
            input.set_cursor(cursor);
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

}