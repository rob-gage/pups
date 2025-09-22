// Copyright Rob Gage 2025

use crate::{
    ComplexResult,
    ParseMode,
    ParseResult
};

pub struct Complex;

impl ParseMode for Complex {

    type Result<E, M, O> = ComplexResult<E, M, O>;

}