// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};

pub struct Choice<PL> (PL);

impl<'a, O, E, M, I, P, const COUNT: usize> Parser<'a, O, E, M, I> for Choice<[P; COUNT]> where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
        let start = input.save_cursor();
        let mut last_error: Option<_Mode::ErrorForm<E>> = None;
        let mut last_messages: Option<_Mode::MessageContainer<M>> = None;
        for parser in &self.0 {
            match parser.apply::<_Mode>(input) {
                Success(output, messages) => return Success(output, messages),
                Failure(error, messages) => {
                    input.restore_cursor(start);
                    last_error = Some(error);
                    last_messages = Some(messages);
                }
            }
        }
        Failure(
            last_error.expect("`choice` must not be used with no parsers"),
            last_messages.expect("`choice` must not be used with no parsers"),
        )
    }

    implement_modes!('a, O, E, M, I);

}

/// Optionally applies a parser, converting a failure into `Option::None`
pub const fn choice<'a, O, E, M, I, PL>(
    parser_list: PL,
) -> impl Parser<'a, O, E, M, I>
where
    I: Input<'a>,
    Choice<PL>: Parser<'a, O, E, M, I>,
{ Choice (parser_list) }