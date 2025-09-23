// Copyright Rob Gage 2025

use std::marker::PhantomData;


/// A results from applying parsers to input
pub struct ParseResult<Error, Message, Output, MessageContainer = Vec<Message>> {
    /// The internal `Result<O, E>` type
    result: Result<Output, Error>,
    /// Stores all the messages that have been accumulated by the parser
    pub messages: MessageContainer,
    _marker: PhantomData<Message>,
}


impl<Error, Message, Output, MessageContainer>
ParseResult<Error, Message, Output, MessageContainer> {

    /// Returns `true` if this `ParseResult` represents a failed parser application
    pub fn is_failure(&self) -> bool { ! self.result.is_ok() }

    /// Returns `true` if this `ParseResult` represents a successful parser application
    pub fn is_success(&self) -> bool { self.result.is_ok() }

}