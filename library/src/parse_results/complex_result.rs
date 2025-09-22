// Copyright Rob Gage 2025

use crate::ParseResult;

/// The result from applying a parser to input, returning output and errors together
pub enum ComplexResult<E, M, O> {
    /// Represents a failed parser application
    Failure (E, Vec<M>),
    /// Represents a successful parser application with non-fatal accumulated errors
    Success (O, Vec<M>)
}

impl<E, M, O> ParseResult<E, M, O> for ComplexResult<E, M, O> {

    type MessageContainer = Vec<M>;

    fn add_message(message_container: &mut Vec<M>, message: M) {
        message_container.push(message);
    }

    fn failure(error: E, messages: Vec<M>) -> Self {
        ComplexResult::Failure (error, messages)
    }

    fn from_result(result: Result<O, E>, messages: Self::MessageContainer) -> Self {
        match result {
            Ok (output) => ComplexResult::Success (output, messages),
            Err (errors) => ComplexResult::Failure (errors, messages),
        }
    }

    fn new_message_container() -> Self::MessageContainer { vec![] }

    fn message_container(self) -> Self::MessageContainer {
        match self {
            ComplexResult::Failure (_, messages) => messages,
            ComplexResult::Success (_, messages) => messages,
        }
    }

    fn success(output: O, messages: Self::MessageContainer) -> Self {
        ComplexResult::Success (output, messages)
    }

    fn to_result(self) -> Result<O, E> {
        match self {
            ComplexResult::Failure (error, _) => Err(error),
            ComplexResult::Success (output, _) => Ok(output)
        }
    }

}