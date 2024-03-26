#[allow(unused_parens)]
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
