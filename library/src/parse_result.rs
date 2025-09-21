// Copyright Rob Gage 2025

/// The result from applying a parser to input, returning output and errors together
pub enum ParseResult<O, E> {
    /// Represents a failed parser application
    Failure (Vec<E>),
    /// Represents a successful parser application with non-fatal accumulated errors
    Success (O, Vec<E>)
}

impl<O, E> ParseResult<O, E> {

    /// Maps the output type of this `ParserResult` to a new type by applying a function to the
    /// output
    pub fn map<_O>(self, f: impl Fn(O) -> _O) -> ParseResult<_O, E> {
        match self {
            Self::Failure (errors) => ParseResult::Failure (errors),
            Self::Success (output, errors) => ParseResult::Success (f(output), errors)
        }
    }

    /// Maps the error type of this `ParserResult` to a new type by applying a function to each of
    /// the errors
    pub fn map_errors<_E>(self, f: impl Fn(E) -> _E) -> ParseResult<O, _E> {
        let mapper = |errors: Vec<E>| errors.into_iter().map(f).collect();
        match self {
            Self::Failure (errors) => ParseResult::Failure (mapper(errors)),
            Self::Success (output, errors) => ParseResult::Success (output, mapper(errors)),
        }
    }

    /// Add accumulated errors to this `ParseResult`
    pub fn with_errors(self, errors: Vec<E>) -> Self {
        match self {
            Self::Failure (mut old_errors) => {
                old_errors.extend(errors);
                Self::Failure (old_errors)
            },
            Self::Success (output, mut old_errors) => {
                old_errors.extend(errors);
                Self::Success (output, old_errors)
            }
        }
    }

}