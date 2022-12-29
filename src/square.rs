use core::fmt::Display;
use core::fmt::Formatter;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum Square {
    None,
    O,
    X,
}

impl Default for Square {
    fn default() -> Self {
        Square::None
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::None => ' ',
                Self::O => 'O',
                Self::X => 'X',
            }
        )
    }
}
