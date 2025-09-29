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
pub struct Token (pub &'static str);

impl<I> Parser<I> for Token
where
    I: Input + TextInput,
{
    type Output = String;

    type Error = ();

    type Message = ();

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ParseResult<String, (), (), _Mode> {
        if input.starts_with(self.0) {
            Success (_Mode::convert_output(self.0.to_string()), _Mode::new_message_container())
        } else {
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

}