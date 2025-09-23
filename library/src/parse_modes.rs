// Copyright Rob Gage 2025

mod parse;

pub use parse::Parse;

/// Implementors represent different modes that parsers run in, each accomplishing different goals
pub trait ParseMode {

    /// The error type produced by this `ParseMode`
    type Error;

    /// The message type produced by this `ParseMode`
    type Message;

    /// The type used to store messages produced by this `ParseMode`
    type MessageContainer;

    /// The output type produced by this `ParseMode`
    type Output;

    /// Adds a `Self::Message` to a `Self::MessageContainer`
    fn add_message_to_container(
        message: Self::Message,
        message_container: &mut Self::MessageContainer
    );

    /// Combine two `Self::MessageContainer`s into one
    fn combine_message_containers(
        a: &mut Self::MessageContainer,
        b: Self::MessageContainer,
    );

    /// Creates a new empty `Self::MessageContainer`
    fn new_message_container() -> Self::MessageContainer;

}