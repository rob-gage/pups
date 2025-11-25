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


/// Macro to automatically implement choice for tuples of various sizes
macro_rules! implement_choice {
    ( $first:ident $( $rest:ident )* ) => {
        choice_tuple!($first $(, $rest)*);
    };
}

macro_rules! choice_tuple {
    ( $first:ident $(, $rest:ident)* ) => {
        impl<'a, O, E, M, I, $first, $( $rest, )*> Parser<'a, O, E, M, I>
        for Choice<($first, $( $rest, )*)>
        where
            I: Input<'a>,
            $first: Parser<'a, O, E, M, I>,
            $( $rest: Parser<'a, O, E, M, I>, )*
        {
            fn apply<_Mode: Mode>(
                &self,
                input: &'a I
            ) -> ModeResult<O, E, M, _Mode> {
                let ($first, $( $rest, )*) = &self.0;
                let mut result: ModeResult<O, E, M, _Mode> = _Mode::apply_parser($first, input);
                $(
                    result = match result {
                        ModeResult::Success(_, _) => return result,
                        ModeResult::Failure(_, _) => _Mode::apply_parser($rest, input),
                    };
                )*
                result
            }

            implement_modes!('a, O, E, M, I);
        }
    }
}

pub struct Choice<PL> (PL);

implement_choice!(P1);
implement_choice!(P1 P2);
implement_choice!(P1 P2 P3);
implement_choice!(P1 P2 P3 P4);
implement_choice!(P1 P2 P3 P4 P5);
implement_choice!(P1 P2 P3 P4 P5 P6);
implement_choice!(P1 P2 P3 P4 P5 P6 P7);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8);

implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15);
implement_choice!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16);

/// Optionally applies a parser, converting a failure into `Option::None`
pub const fn choice<'a, O, E, M, I, PL>(
    parser_list: PL,
) -> impl Parser<'a, O, E, M, I>
where
    I: Input<'a>,
    Choice<PL>: Parser<'a, O, E, M, I>,
{ Choice (parser_list) }