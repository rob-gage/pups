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

/// Parses a lexical token
pub struct Whitespace;

impl<C, I> Parser<I> for Whitespace
where
    C: Character,
    I: Input<Item = C> + TextInput,
{
    type Output = Vec<C>;

    type Error = ();

    type Message = ();

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<Vec<C>, (), (), _Mode> {
        let mut whitespace: _Mode::OutputForm<Vec<C>> = _Mode::convert_output(Vec::new());
        let mut not_empty: bool = false;
        loop {
            let cursor: usize = input.cursor();
            let Some (character) = input.next() else { break };
            if !character.is_whitespace() { input.set_cursor(cursor); break }
            whitespace = _Mode::merge_outputs(
                whitespace,
                _Mode::convert_output(character),
                |mut ws, c| { ws.push(c); ws }
            );
            not_empty = true;
        }
        if not_empty { Success (whitespace, _Mode::new_message_container()) }
        else { Failure (_Mode::convert_error(()), _Mode::new_message_container())}
    }

}