// Copyright Rob Gage 2025

mod choice;
mod end;
mod first_match;
mod iterated;
mod mapped;
mod nothing;
mod optional;
mod recoverable;
mod sequenced;
mod mapped_messages;
mod mapped_error;

use crate::{
    Check,
    Input,
    Mode,
    Parse,
    ModeResult,
};
use std::marker::PhantomData;

// pub use choice::Choice;
pub use end::end;
pub use first_match::seek;
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

    /// Parses input, returning a fully detailed result
    fn parse(&self, input: &'a I) -> ModeResult<O, E, M, Parse>;

}

impl<'a, O, E, M, F, I> Parser<'a, O, E, M, I> for F
where
    F: Fn(&'a I) -> ModeResult<O, E, M, Parse>,
    I: Input<'a> +'a
{

    fn apply<_Mode: Mode>(&self, input: &'a I) -> ModeResult<O, E, M, _Mode> {
        _Mode::apply_parser(self, input)
    }

    fn check(&self, input: &'a I) -> bool { self.parse(input).is_success() }

    fn parse(&self, input: &'a I) -> ModeResult<O, E, M, Parse> { self(input) }

}