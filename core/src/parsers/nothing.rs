// Copyright Rob Gage 2025

use crate::{
    implement_modes,
    Input,
    Mode,
    ModeResult::{
        self,
        Success,
    },
    Parser,
};
use std::marker::PhantomData;

pub struct Nothing<E, M> (PhantomData<(E, M)>);

impl<'a, E, M, I> Parser<'a, (), E, M, I> for Nothing<E, M>
where
    I: Input<'a>,
{

    fn apply<_Mode: Mode>(&self, _: &'a I) -> ModeResult<(), E, M, _Mode> {
        Success (_Mode::convert_output(()), _Mode::new_message_container())
    }

    implement_modes!('a, (), E, M, I);

}

/// Parses absolutely nothing
pub const fn nothing<'a, E, M, I>() -> impl Parser<'a, (), E, M, I>
where
    I: Input<'a>,
{ Nothing (PhantomData) }