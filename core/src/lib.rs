// Copyright Rob Gage 2025

mod input;
mod modes;
mod parse_result;
mod parsers;

use modes::*;

pub mod prelude {
    pub use crate::{
        parse_result::ParseResult,
        parsers::*,
    };
}

pub use prelude::*;
pub use modes::Mode;
pub use input::Input;