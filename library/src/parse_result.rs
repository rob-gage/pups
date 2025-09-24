// Copyright Rob Gage 2025

use crate::Mode;

/// A result produced by applying a parser to input
pub enum ParseResult<Output, Error, Message, _Mode>
where
    _Mode: Mode,
{
    /// Represents a successful parser application
    Success (_Mode::ErrorForm<Error>, _Mode::MessageContainer<Message>),
    /// Represents a failed parser application
    Failure (_Mode::OutputForm<Output>, _Mode::MessageContainer<Message>),
}


impl<Output, Error, Message, _Mode> ParseResult<Output, Error, Message, _Mode>
where
    _Mode: Mode
{

    /// Returns `true` if this `ParseResult` represents a successful parser application
    pub const fn is_success(&self) -> bool { matches!(self, ParseResult::Success(..)) }

    /// Returns `true` if this `ParseResult` represents a failed parser application
    pub const fn is_failure(&self) -> bool { matches!(self, ParseResult::Failure(..)) }

}