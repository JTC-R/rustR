#[allow(unused_parens)]
#[allow(non_snake_case)]




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
