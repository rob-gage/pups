// Copyright Rob Gage 2025

use crate::{
    Input,
    Mode,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};
use std::marker::PhantomData;


/// Parses absolutely nothing
pub struct Nothing<E, M> (pub PhantomData<(E, M)>);

impl<'a, E, M, I> Parser<'a, (), E, M, I> for Nothing<E, M>
where
    I: Input<'a>,
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<(), E, M, _Mode> {
        Success (_Mode::convert_output(()), _Mode::new_message_container())
    }

}