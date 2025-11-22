// Copyright Rob Gage 2025

mod boxed;
mod choice;
mod end;
mod emitting;
mod first;
mod iterated;
mod mapped;
mod nothing;
mod optional;
mod recoverable;
mod sequenced;
mod mapped_messages;
mod mapped_error;
mod traced;

use crate::{
    Check,
    Input,
    Mode,
    Verbose,
    ModeResult,
};
use std::marker::PhantomData;

pub use boxed::boxed;
pub use choice::choice;
pub use emitting::emitting;
pub use end::end;
pub use first::first;
pub use iterated::{
    many,
    separated,
};
pub use mapped::mapped;
pub use mapped_error::mapped_error;
pub use mapped_messages::mapped_messages;
pub use nothing::nothing;
pub use optional::optional;
pub use recoverable::recoverable;
pub use sequenced::{
    delimited,
    preceded,
    sequenced,
    terminated,
};
pub use traced::traced;

/// Implementors can be parsed from an input type
pub trait Parser<'a, O, E, M, I>
where
    I: Input<'a>
{

    /// Applies a parser
    fn apply<_Mode: Mode>(
        &self,
        input: &'a I
    ) -> ModeResult<O, E, M, _Mode>
    where
        Self: Sized;

    /// Checks input, returning a boolean if it matches this parser
    fn check(&self, input: &'a I) -> bool;
    
    /// Parses input, returning an output or error
    fn parse(&self, input: &'a I) -> Result<O, E>;

    /// Parses input, returning a fully detailed result with messages
    fn parse_verbose(&self, input: &'a I) -> ModeResult<O, E, M, Verbose>;

}

impl<'a, O, E, M, F, I> Parser<'a, O, E, M, I> for F
where
    F: Fn(&'a I) -> ModeResult<O, E, M, Verbose>,
    I: Input<'a> +'a
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
        _Mode::apply_parser(self, input)
    }

    fn check(&self, input: &'a I) -> bool { self.parse_verbose(input).is_success() }

    fn parse(&self, input: &'a I) -> Result<O, E> { self.parse_verbose(input).to_result() }

    fn parse_verbose(&self, input: &'a I) -> ModeResult<O, E, M, Verbose> { self(input) }

}