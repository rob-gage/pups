// Copyright Rob Gage 2025

use crate::{
    Input,
    mapped,
    Parser,
};


/// Applies a parser that becomes `Some (_)` when it works, and `None` when it doesn't
pub const fn optional<E, I, O, P>(parser: P)-> impl Parser<I, Error = E, Output = Option<O>> where
    I: Input,
    P: Parser<I, Output = O, Error = E>,
{
    mapped(parser, |result| match result {
        Ok (output) => Ok (Some (output)),
        Err (errors) => Ok (None)
    })
}