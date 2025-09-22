// Copyright Rob Gage 2025

mod complex_result;

pub use complex_result::ComplexResult;

/// Implementors represent results from applying parsers to input
pub trait ParseResult<E, M, O> {

    /// The type used to store the messages contained by this result
    type MessageContainer;

    /// Adds a message to a `MessageContainer`
    fn add_message(message_container: &mut Self::MessageContainer, message: M);

    /// Creates a result that represents the failed application of a parser
    fn failure(error: E, messages: Self::MessageContainer) -> Self;

    /// Creates a new result from a `Result<O, E>` and messages
    fn from_result(result: Result<O, E>, messages: Self::MessageContainer) -> Self;

    /// Returns the messages contained by this result
    fn message_container(self) -> Self::MessageContainer;

    /// Creates a `MessageContainer` for this result type
    fn new_message_container() -> Self::MessageContainer;

    /// Creates a result that represents the successful application of a parser
    fn success(output: O, messages: Self::MessageContainer) -> Self;

    /// Converts this result to a `Result<O, E>`
    fn to_result(self) -> Result<O, E>;

}