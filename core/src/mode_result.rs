// Copyright Rob Gage 2025

use crate::Mode;

/// A result produced by applying a parser to input
pub enum ModeResult<O, E, M, _Mode>
where
    _Mode: Mode,
{
    /// Represents a successful parser application
    Success (_Mode::OutputForm<O>, _Mode::MessageContainer<M>),
    /// Represents a failed parser application
    Failure (_Mode::ErrorForm<E>, _Mode::MessageContainer<M>),
}

impl<Output, Error, Message, _Mode> ModeResult<Output, Error, Message, _Mode>
where
    _Mode: Mode
{

    /// Returns `true` if this result represents a successful parser application
    pub const fn is_success(&self) -> bool { matches!(self, ModeResult::Success(..)) }

    /// Returns `true` if this result represents a failed parser application
    pub const fn is_failure(&self) -> bool { matches!(self, ModeResult::Failure(..)) }

    /// Returns this `ModeResult` as a `Result`
    pub fn to_result(self) -> Result<_Mode::OutputForm<Output>, _Mode::ErrorForm<Error>> {
        match self {
            ModeResult::Success (output, _) => Ok (output),
            ModeResult::Failure (error, _) => Err (error),
        }
    }

}