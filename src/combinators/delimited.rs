// Copyright Rob Gage 2025

use crate::{
    InputStream,
    Parser,
};
use super::{
    preceded,
    terminated,
};

/// Wrap a parser in delimiters
pub const fn delimited<E, I, OA, OB, OC, PA, PB, PC>(
    opening_delimiter: PA,
    parser: PB,
    closing_delimiter: PC,
) -> impl Parser<I, Error = E, Output = OB>
where
    I: InputStream,
    PA: Parser<I, Output = OA, Error = E>,
    PB: Parser<I, Output = OB, Error = E>,
    PC: Parser<I, Output = OC, Error = E>,
{
    preceded(opening_delimiter, terminated(parser, closing_delimiter))
}