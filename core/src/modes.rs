// Copyright Rob Gage 2025

use crate::{
    Input,
    ModeResult::{
        self,
        Failure,
        Success,
    },
    Parser,
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

    /// Applies a parser using this `Mode`
    fn apply_parser<'a, O, E, M, I, P>(
        parser: P,
        input: &'a I,
    ) -> ModeResult<O, E, M, Self>
    where
        I: Input<'a>,
        P: Parser<'a, O, E, M, I>;

    /// Converts an output to its representational form in this mode
    fn convert_output<O>(output: impl Into<O>) -> Self::OutputForm<O>;

    /// Converts an error to its representational form in this mode
    fn convert_error<E>(error: impl Into<E>) -> Self::ErrorForm<E>;

    /// Merges two output types into one using a function
    fn merge_outputs<OA1, OA2, OB>(
        output_1: Self::OutputForm<OA1>,
        output_2: Self::OutputForm<OA2>,
        function: impl Fn(OA1, OA2) -> OB,
    ) -> Self::OutputForm<OB>;

    /// Merges two error types into one using a function
    fn merge_errors<EA1, EA2, EB>(
        error_1: Self::ErrorForm<EA1>,
        error_2: Self::ErrorForm<EA2>,
        function: impl Fn(EA1, EA2) -> EB,
    ) -> Self::ErrorForm<EB>;

    /// Combine two `Self::MessageContainer`s
    fn merge_message_containers<M>(
        a: Self::MessageContainer<M>,
        b: impl Into<Self::MessageContainer<M>>
    ) -> Self::MessageContainer<M>;

    /// Maps a result's output type to another type in this mode
    fn map_output<OA, OB, E, M>(
        result: ModeResult<OA, E, M, Self>,
        function: impl Fn(OA) -> OB,
    ) -> ModeResult<OB, E, M, Self>;

    /// Maps a result's error type to another type in this mode
    fn map_error<O, EA, EB, M>(
        result: ModeResult<O, EA, M, Self>,
        function: impl Fn(EA) -> EB,
    ) -> ModeResult<O, EB, M, Self>;

    /// Maps a result's message type to another type in this mode
    fn map_messages<O, E, MA, MB>(
        result: ModeResult<O, E, MA, Self>,
        function: impl Fn(MA) -> MB,
    ) -> ModeResult<O, E, MB, Self>;

    /// Create a new empty `Self::MessageContainer`
    fn new_message_container<M>() -> Self::MessageContainer<M>;

    /// Adds a message to a `Self::MessageContainer`
    fn add_message_to_container<M>(container: &mut Self::MessageContainer<M>, message: M);

}


/// Parser mode that does not extract any information, just ensures the parser matches the input
pub struct Check;

impl Mode for Check {

    type OutputForm<O> = ();

    type ErrorForm<E> = ();

    type MessageContainer<M> = ();

    fn apply_parser<'a, O, E, M, I, P>(parser: P, input: &'a I) -> ModeResult<O, E, M, Self>
    where
        I: Input<'a, >,
        P: Parser<'a, O, E, M, I>,
    { if parser.check(input) { Success ((), ()) } else { Failure ((), ()) } }

    fn convert_output<O>(_: impl Into<O>) -> () { () }

    fn convert_error<E>(_: impl Into<E> ) -> () { () }

    fn merge_outputs<OA1, OA2, OB>(
        _: (),
        _: (),
        _: impl Fn(OA1, OA2) -> OB
    ) -> Self::OutputForm<OB> { () }

    fn merge_errors<EA1, EA2, EB>(
        _: (),
        _: (),
        _: impl Fn(EA1, EA2) -> EB,
    ) -> Self::ErrorForm<EB> { () }

    fn merge_message_containers<M>(
        _: Self::MessageContainer<M>,
        _: impl Into<Self::MessageContainer<M>>
    ) { () }

    fn map_output<OA, OB, E, M>(
        result: ModeResult<OA, E, M, Self>,
        _: impl Fn(OA) -> OB,
    ) -> ModeResult<OB, E, M, Self> {
        if result.is_success() {Success((), ()) } else { Failure((), ()) }
    }

    fn map_error<O, EA, EB, M>(
        result: ModeResult<O, EA, M, Self>,
        _: impl Fn(EA) -> EB,
    ) -> ModeResult<O, EB, M, Self> {
        if result.is_success() {Success((), ()) } else { Failure((), ()) }
    }

    fn map_messages<O, E, MA, MB>(
        result: ModeResult<O, E, MA, Self>,
        _: impl Fn(MA) -> MB,
    ) -> ModeResult<O, E, MB, Self> {
        if result.is_success() {Success((), ()) } else { Failure((), ()) }
    }

    fn new_message_container<M>() -> () { () }

    fn add_message_to_container<M>(_: &mut Self::MessageContainer<M>, _: M) {}

}


/// Parser mode that extracts all information from the parser's input
pub struct Parse;

impl Mode for Parse {

    type OutputForm<O> = O;

    type ErrorForm<E> = E;

    type MessageContainer<M> = Vec<M>;

    fn apply_parser<'a, O, E, M, I, P>(parser: P, input: &'a I) -> ModeResult<O, E, M, Self>
    where
        I: Input<'a>,
        P: Parser<'a, O, E, M, I>,
    { parser.parse(input) }

    fn convert_output<O>(output: impl Into<O>) -> O { output.into() }

    fn convert_error<E>(error: impl Into<E>) -> E  { error.into() }

    fn merge_outputs<OA1, OA2, OB>(
        output_1: OA1,
        output_2: OA2,
        function: impl Fn(OA1, OA2) -> OB
    ) -> OB { Self::convert_output(function(output_1, output_2)) }

    fn merge_errors<EA1, EA2, EB>(
        error_1: EA1,
        error_2: EA2,
        function: impl Fn(EA1, EA2) -> EB
    ) -> EB { Self::convert_error(function(error_1, error_2)) }

    fn merge_message_containers<M>(
        a: Vec<M>,
        b: impl Into<Vec<M>>
    ) -> Vec<M> {
        let mut a: Vec<M> = a;
        a.extend(b.into());
        a
    }

    fn map_output<OA, OB, E, M>(
        result: ModeResult<OA, E, M, Self>,
        function: impl Fn(OA) -> OB,
    ) -> ModeResult<OB, E, M, Self> {
        match result {
            Success (output, messages) => Success (function(output), messages),
            Failure (error, messages) => Failure (error, messages),
        }
    }

    fn map_error<O, EA, EB, M>(
        result: ModeResult<O, EA, M, Self>,
        function: impl Fn(EA) -> EB,
    ) -> ModeResult<O, EB, M, Self> {
        match result {
            Success (output, messages) => Success (output, messages),
            Failure (error, messages) => Failure (function(error), messages),
        }
    }

    fn map_messages<O, E, MA, MB>(
        result: ModeResult<O, E, MA, Self>,
        function: impl Fn(MA) -> MB,
    ) -> ModeResult<O, E, MB, Self> {
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

}