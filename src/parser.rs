// Copyright Rob Gage 2025

use crate::InputStream;

/// Implementors can be parsed from an input type
pub trait Parser<I>
where I
      : InputStream
{

    /// The type for errors that can occur with this parser
    type Error;

    /// The output type that is parsed by this parser
    type Output;

    /// Parses an `InputStream` to return `Self::Output` or `Vec<Self::Error>`
    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>>;

}