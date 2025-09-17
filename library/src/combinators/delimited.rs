// Copyright Rob Gage 2025

use crate::{
    Input,
    Parser,
    preceded,
    terminated
};


/// Parses input between a prefix and a terminator
pub const fn delimited<E, I, O1, O2, O3, P1, P2, P3>(
    prefix: P1,
    parser: P2,
    terminator: P3,
) -> impl Parser<I, Error = E, Output = O2>
where
    I: Input,
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
    P3: Parser<I, Output = O3, Error = E>,
{ preceded(prefix, terminated(parser, terminator)) }