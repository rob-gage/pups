// Copyright Rob Gage 2025

use crate::Input;

/// Implementors can be parsed from an input type
pub trait Parser<I> where
    I: Input
{

    /// The type for errors that can occur with this parser
    type Error;

    /// The output type that is parsed by this parser
    type Output;

    /// Consumes `Input` and returns true if it can be accepted by this parser
    fn accept(&self, input: &mut I) -> bool { self.parse(input).is_ok() }

    /// Parses a `Input` to return `Self::Output` or `Vec<Self::Error>`
    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>>;

}