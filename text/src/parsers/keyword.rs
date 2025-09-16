// Copyright Rob Gage 2025

use crate::{
    Character,
    TextInput
};
use pups::{
    Input,
    Parser,
};

/// See `keyword`
struct Keyword (String);

impl<I, T> Parser<I> for Keyword where
    I: Input<Item = T> + TextInput,
    T: Character,
{

    type Output = ();

    type Error = ();

    fn accept(&self, input: &mut I) -> bool {
        if input.starts_with(&self.0) {
            input.skip_bytes(self.0.len()); true
        } else { false }
    }

    fn parse(&self, input: &mut I) -> Result<Self::Output, Vec<Self::Error>> {
        if self.accept(input) { Ok (()) } else { Err (vec![]) }
    }

}

/// Parses a keyword
pub fn keyword(keyword: &str) -> Keyword { Keyword (keyword.to_string()) }