#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// TODO Document error at
// https://github.com/rustwasm/wasm-bindgen/issues/new?assignees=&labels=bug&template=bug-report---.md&title=
/*
pub enum Error {
    BadThing,
}
*/

// TODO Pick a better name than MyError (just not Error).
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MyError {
    // This is not the current board
    #[error("Wrong board")]
    WrongBoard,

    #[error("Invalid Board Position")]
    InvalidBoard,

    #[error("Invalid Square Position")]
    InvalidSquare,

    #[error("Square has already been played")]
    AlreadyPlayed,

    #[error("Board has already been won")]
    AlreadyWon,
}
