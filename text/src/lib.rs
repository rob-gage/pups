// Copyright Rob Gage 2025

mod character;
mod text;
mod parsers;
mod text_input;

use character::Character;
use text_input::TextInput;

pub mod prelude {
    pub use crate::{
        parsers::*,
        text::Text,
    };
}
pub use prelude::*;