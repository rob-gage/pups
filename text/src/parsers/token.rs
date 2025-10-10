// Copyright Rob Gage 2025

use crate::TextInput;
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
pub struct Token (pub &'static str);

impl<'a, I> Parser<'a, I> for Token
where
    I: Input<'a> + TextInput,
{
    type Output = &'a str;

    type Error = ();

    type Message = ();

    fn apply<_Mode: Mode>(
        &self,
        input: &mut I
    ) -> ModeResult<&'a str, (), (), _Mode> {
        if input.starts_with(self.0) {
            Success (_Mode::convert_output(self.0), _Mode::new_message_container())
        } else {
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

}