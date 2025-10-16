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

impl<'a, I> Parser<'a, &'static str, (), (), I> for Token
where
    I: Input<'a> + TextInput,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<&'static str, (), (), _Mode> {
        if input.starts_with(self.0) {
            Success (_Mode::convert_output(self.0), _Mode::new_message_container())
        } else {
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

}