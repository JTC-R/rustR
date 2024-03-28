#[allow(unused_parens)]
#[allow(non_snake_case)]
use crate::tokenize::{TokeError, TokeErrType};



pub enum TokenizeErrorType {
    SpaceError,
    AlphaError,
    NumError,
    PunctError,

    SpaceUnexpectedToken,
    AlphaUnexpectedToken,
    NumUnexpectedToken,
    PunctErrorUnexpectedToken,
}

pub struct TokenizeError {
    pub err: TokenizeErrorType,
}
