// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput
};
use pups_core::{
    Input,
    Mode,
    ParseResult::{
        self,
        Failure,
        Success,
    },
    Parser
};

/// Parses whitespace
pub struct Whitespace;

impl<C, I> Parser<I> for Whitespace
where
    C: Character,
    I: Input<Item = C> + TextInput,
{
    type Output = String;

    type Error = ();

    type Message = ();

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<String, (), (), _Mode> {
        let mut whitespace: _Mode::OutputForm<String> = _Mode::convert_output(String::new());
        let mut not_empty: bool = false;
        loop {
            let cursor: usize = input.cursor();
            let Some (character) = input.next() else { break };
            if !character.is_whitespace() { input.set_cursor(cursor); break }
            whitespace = _Mode::merge_outputs(
                whitespace,
                _Mode::convert_output(character),
                |mut ws, c: C| { c.write(&mut ws); ws }
            );
            not_empty = true;
        }
        if not_empty { Success (whitespace, _Mode::new_message_container()) }
        else { Failure (_Mode::convert_error(()), _Mode::new_message_container())}
    }

}