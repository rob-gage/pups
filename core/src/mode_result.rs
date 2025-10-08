// Copyright Rob Gage 2025

use crate::Mode;

/// A result produced by applying a parser to input
pub enum ModeResult<Output, Error, Message, _Mode>
where
    _Mode: Mode,
{
    /// Represents a successful parser application
    Success (_Mode::OutputForm<Output>, _Mode::MessageContainer<Message>),
    /// Represents a failed parser application
    Failure (_Mode::ErrorForm<Error>, _Mode::MessageContainer<Message>),
}


impl<Output, Error, Message, _Mode> ModeResult<Output, Error, Message, _Mode>
where
    _Mode: Mode
{

    /// Returns `true` if this `ParseResult` represents a successful parser application
    pub const fn is_success(&self) -> bool { matches!(self, ModeResult::Success(..)) }

    /// Returns `true` if this `ParseResult` represents a failed parser application
    pub const fn is_failure(&self) -> bool { matches!(self, ModeResult::Failure(..)) }

}