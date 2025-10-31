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

/// Parses a unicode identifier
struct UnicodeIdentifier;

impl<'a, C, I> Parser<'a, I::Slice, (), (), I> for UnicodeIdentifier
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<I::Slice, (), (), _Mode> {
        let start: usize = input.save_cursor();
        if let Some (character) = input.next() && character.is_unicode_identifier_start() {
            loop {
                let cursor: usize = input.save_cursor();
                let Some (character) = input.next() else { break; };
                if !character.is_unicode_identifier_continuation() {
                    input.restore_cursor(cursor);
                    break;
                };
            }
            let length: usize = input.save_cursor() - start;
            Success (
                _Mode::convert_output(input.consume(length).unwrap()),
                _Mode::new_message_container()
            )
        } else { Failure (_Mode::convert_error(()), _Mode::new_message_container()) }
    }

    implement_modes!('a, I::Slice, (), (), I);

}

/// Parses a unicode identifier
pub const fn unicode_identifier<'a, C, I>(
) -> impl Parser<'a, I::Slice, (), (), I>
where
    C: Character,
    I: Input<'a, Item = C> + TextInput,
{ UnicodeIdentifier }