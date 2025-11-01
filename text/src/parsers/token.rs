// Copyright Rob Gage 2025

use crate::TextInput;
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

struct Token (&'static str);

impl<'a, I> Parser<'a, I::Slice, (), (), I> for Token
where
    I: Input<'a> + TextInput,
{

    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<I::Slice, (), (), _Mode> {
        let start: usize = input.store_cursor();
        if input.starts_with(self.0) {
            input.skip_bytes(self.0.len());
            Success (
                _Mode::convert_output(input.slice(start, input.store_cursor())),
                _Mode::new_message_container()
            )
        } else {
            Failure (_Mode::convert_error(()), _Mode::new_message_container())
        }
    }

    implement_modes!('a, I::Slice, (), (), I);

}

/// Parses a lexical token
pub const fn token<'a, I>(
    lexeme: &'static str
) -> impl Parser<'a, I::Slice, (), (), I>
where
    I: Input<'a> + TextInput,
{ Token (lexeme) }