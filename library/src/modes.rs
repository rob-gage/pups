// Copyright Rob Gage 2025

use crate::ParseResult::{
    self,
    Failure,
    Success,
};


/// Implementors represent different modes that parsers run in, each accomplishing different goals
pub trait Mode
where
    Self: Sized,
{

    /// The representational form of a successful parser application in this mode
    type OutputForm<O>;

    /// The representational form of a failed parser application in this mode
    type ErrorForm<E>;

    /// The representational form of stored messages used by this mode
    type MessageContainer<M>;

    /// Converts an output to its representational form in this mode
    fn convert_output<O>(output: O) -> Self::OutputForm<O>;

    /// Converts an error to its representational form in this mode
    fn convert_error<E>(error: E) -> Self::ErrorForm<E>;

    /// Maps a result's output type to another type in this mode
    fn map_output<OA, OB, E, M>(
        result: ParseResult<OA, E, M, Self>,
        function: impl Fn(OA) -> OB,
    ) -> ParseResult<OB, E, M, Self>;

    /// Maps a result's error type to another type in this mode
    fn map_error<O, EA, EB, M>(
        result: ParseResult<O, EA, M, Self>,
        function: impl Fn(EA) -> EB,
    ) -> ParseResult<O, EB, M, Self>;

    /// Maps a result's message type to another type in this mode
    fn map_messages<O, E, MA, MB>(
        result: ParseResult<O, E, MA, Self>,
        function: impl Fn(MA) -> MB,
    ) -> ParseResult<O, E, MB, Self>;

    /// Create a new empty `Self::MessageContainer`
    fn new_message_container<M>() -> Self::MessageContainer<M>;

    /// Adds a message to a `Self::MessageContainer`
    fn add_message_to_container<M>(container: &mut Self::MessageContainer<M>, message: M);

    /// Combine two `Self::MessageContainer`s
    fn combine_message_containers<M>(
        a: &mut Self::MessageContainer<M>,
        b: impl Into<Self::MessageContainer<M>>
    );

}


/// Parser mode that does not extract any information, just ensures the parser matches the input
pub struct Check;

impl Mode for Check {

    type OutputForm<O> = ();

    type ErrorForm<E> = ();

    type MessageContainer<M> = ();

    fn convert_output<O>(_: O) -> () { () }

    fn convert_error<E>(_: E ) -> () { () }

    fn map_output<OA, OB, E, M>(
        result: ParseResult<OA, E, M, Self>,
        _: impl Fn(OA) -> OB,
    ) -> ParseResult<OB, E, M, Self> {
        if result.is_success() {Success((), ()) } else { Failure((), ()) }
    }

    fn map_error<O, EA, EB, M>(
        result: ParseResult<O, EA, M, Self>,
        _: impl Fn(EA) -> EB,
    ) -> ParseResult<O, EB, M, Self> {
        if result.is_success() {Success((), ()) } else { Failure((), ()) }
    }

    fn map_messages<O, E, MA, MB>(
        result: ParseResult<O, E, MA, Self>,
        _: impl Fn(MA) -> MB,
    ) -> ParseResult<O, E, MB, Self> {
        if result.is_success() {Success((), ()) } else { Failure((), ()) }
    }

    fn new_message_container<M>() -> () { () }

    fn add_message_to_container<M>(_: &mut Self::MessageContainer<M>, _: M) {}

    fn combine_message_containers<M>(
        _: &mut Self::MessageContainer<M>,
        _: impl Into<Self::MessageContainer<M>>
    ) { }

}


/// Parser mode that extracts all information from the parser's input
pub struct Parse;

impl Mode for Parse {

    type OutputForm<O> = O;

    type ErrorForm<E> = E;

    type MessageContainer<M> = Vec<M>;

    fn convert_output<O>(output: O) -> O { output }

    fn convert_error<E>(error: E) -> E  { error }

    fn map_output<OA, OB, E, M>(
        result: ParseResult<OA, E, M, Self>,
        function: impl Fn(OA) -> OB,
    ) -> ParseResult<OB, E, M, Self> {
        match result {
            Success (output, messages) => Success (function(output), messages),
            Failure (error, messages) => Failure (error, messages),
        }
    }

    fn map_error<O, EA, EB, M>(
        result: ParseResult<O, EA, M, Self>,
        function: impl Fn(EA) -> EB,
    ) -> ParseResult<O, EB, M, Self> {
        match result {
            Success (output, messages) => Success (output, messages),
            Failure (error, messages) => Failure (function(error), messages),
        }
    }

    fn map_messages<O, E, MA, MB>(
        result: ParseResult<O, E, MA, Self>,
        function: impl Fn(MA) -> MB,
    ) -> ParseResult<O, E, MB, Self> {
        match result {
            Success(output, messages) => Success(output, messages.into_iter()
                .map(function).collect::<Vec<MB>>()),
            Failure(error, messages) => Failure(error, messages.into_iter()
                .map(function).collect::<Vec<MB>>()),
        }
    }

    fn new_message_container<M>() -> Vec<M> { Vec::new() }

    fn add_message_to_container<M>(container: &mut Self::MessageContainer<M>, message: M) {
        container.push(message);
    }

    fn combine_message_containers<M>(
        a: &mut Self::MessageContainer<M>,
        b: impl Into<Self::MessageContainer<M>>
    ) { a.extend(b.into()); }

}