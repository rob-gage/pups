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

impl<E, M> Nothing<E, M> {

    /// Creates a new `Nothing`
    pub const fn new() -> Self { Self(PhantomData) }

}

impl<E, M, I> Parser<I> for Nothing<E, M>
where
    I: Input,
{

    type Output = ();

    type Error = E;

    type Message = M;

    fn apply<_Mode: Mode>(&self, input: &mut I) -> ModeResult<(), E, M, _Mode> {
        Success (_Mode::convert_output(()), _Mode::new_message_container())
    }

}