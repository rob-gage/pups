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

// struct Boxed<'a, O, E, M, I> (Box<dyn Parser<'a, O, E, M, I> + 'a>);
// 
// impl<'a, O, E, M, I> Parser<'a, O, E, M, I> for Boxed<'a, O, E, M, I>
// where
//     I: Input<'a>,
// {
// 
//     fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
//         _Mode::apply_parser(&*self.0, input)
//     }
// 
//     implement_modes!('a, O, E, M, I);
// 
// }

/// Optionally applies a parser, converting a failure into `Option::None`
pub fn boxed<'a, O, E, M, I, P>(
    parser: P,
) -> Box<dyn Parser<'a, O, E, M, I> + 'a>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I> + 'a,
{ Box::new(parser) }