// Copyright Rob Gage 2025

mod combinators;
mod input;
mod mode_result;
mod modes;
mod parsers;
mod macros;

pub mod prelude {

    pub use crate::{
        combinators::Combinators,
        mode_result::ModeResult,
        modes::{
            Check,
            Parse,
            Verbose,
        },
        parsers::*,
    };

    /// Shorthand result type for parsers
    pub type ParseResult<O, E = (), M = ()> = ModeResult<O, E, M, Verbose>;

}

pub use input::Input;
pub use prelude::*;
pub use modes::Mode;