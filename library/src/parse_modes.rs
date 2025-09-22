// Copyright Rob Gage 2025

mod complex;

/// Implementors represent different modes that parsers run in, each accomplishing different goals
pub trait ParseMode {

    /// The result type returned by a parser being used in this mode
    type Result<E, M, O>;

}