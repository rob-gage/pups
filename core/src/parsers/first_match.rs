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

struct FirstMatch<P> (P);

impl<'a, O, E, M, I, P> Parser<'a, Option<O>, E, M, I> for FirstMatch<P>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<Option<O>, E, M, _Mode> {
        while let Some(_) = input.peek() {
            match self.0.apply::<_Mode>(input) {
                Failure (_, _) => input.advance(),
                success => return _Mode::map_output(success, |output| Some (output)),
            }
        }
        Success (_Mode::convert_output(None), _Mode::new_message_container())
    }

    implement_modes!('a, Option<O>, E, M, I);

}

/// Consumes input until a parser can be applied successfully or there is no input left
pub const fn seek<'a, O, E, M, I, P>(
    parser: P,
) -> impl Parser<'a, Option<O>, E, M, I>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{ FirstMatch (parser) }