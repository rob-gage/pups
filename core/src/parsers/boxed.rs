// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
};

/// Optionally applies a parser, converting a failure into `Option::None`
pub fn boxed<'a, O, E, M, I, P>(
    parser: P,
) -> Box<dyn Parser<'a, O, E, M, I> + 'a>
where
    I: Input<'a>,
    P: Parser<'a, O, E, M, I> + 'a,
{ Box::new(parser) }