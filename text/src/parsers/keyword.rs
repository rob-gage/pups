// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput
};
use pups::{
    Input,
    ParseResult::{
        self,
        Failure,
        Success,
    },
    Parser,
};

/// See `keyword`
struct Keyword (String);

impl<I, T> Parser<I> for Keyword where
    I: Input<Item = T> + TextInput,
    T: Character,
{

    type Error = ();

    type Output = ();

    fn parse(&self, input: &mut I) -> ParseResult<Self::Output, Self::Error> {
        if input.starts_with(&self.0) {
            input.skip_bytes(self.0.len());
            Success ((), vec![])
        } else { Failure (vec![]) }
    }

}

/// Parses a keyword
pub fn keyword(keyword: &str) -> Keyword { Keyword (keyword.to_string()) }