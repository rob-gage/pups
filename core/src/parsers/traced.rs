// Copyright Rob Gage 2025

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
    Parse,
    Parser,
};
use std::fmt::Debug;

struct Traced<P> {
    parser: P,
    name: &'static str,
}

impl<'a, O, E, M, I, P> Parser<'a, O, E, M, I> for Traced<P>
where
    O: Debug,
    E: Debug,
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
        _Mode::apply_parser(self, input)
    }

    fn check(&self, input: &'a I) -> bool { self.parse(input).is_success() }

    fn parse(&self, input: &'a I) -> ModeResult<O, E, M, Parse> {
        match self.parser.parse(input) {
            Success (output, messages) => {
                println!("{} successfully parsed output: {:?}", self.name, output);
                Success (output, messages)
            }
            Failure (error, message) => {
                eprintln!("{} encountered error: {:?}", self.name, error);
                Failure (error, message)
            }
        }
    }

}

/// Applies a parser and returns its error or output as a status message with a name
pub const fn traced<'a, O, E, M, I, P>(
    parser: P,
    name: &'static str,
) -> impl Parser<'a, O, E, M, I>
where
    O: Debug,
    E: Debug,
    I: Input<'a>,
    P: Parser<'a, O, E, M, I>,
{ Traced { parser, name } }