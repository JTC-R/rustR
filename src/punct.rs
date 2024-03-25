#[allow(unused_parens)]
use std::thread::current;
use crate::tokenize::{Token, TokenType, start_string_single, start_dbl_string, concat_value, push_to_main};
use crate::tokenize::{TokeError, TokeErrType};

pub fn is_punct(current_chr: char) -> bool {
    if (
        current_chr == '='  ||
        current_chr == '-'  ||
        current_chr == '>'  ||
        current_chr == '<'  ||
        current_chr == '%'  ||
        current_chr == '.'  ||
        current_chr == ','  ||
        current_chr == '_'  ||
        current_chr == '@'  ||
        current_chr == '('  ||
        current_chr == ')'  ||
        current_chr == '{'  ||
        current_chr == '}'  ||
        current_chr == '\\'
    ) {
        return true
    } else {
        return false
    }
}

pub fn match_punct(current_chr: char) -> TokenType {
    if current_chr == '=' {
        return TokenType::SignEq
    } else if current_chr == '-' {
        return TokenType::SignMinus
    } else if current_chr == '>' {
        return TokenType::SignGt
    } else if current_chr == '<' {
        return TokenType::SignLt
    } else if current_chr == '%' {
        return TokenType::SignPrcnt
    } else if current_chr == '.' {
        return TokenType::SignPeriod
    } else if current_chr == ',' {
        return TokenType::SignComma
    } else if current_chr == '_' {
        return TokenType::SignUnderScore
    } else if current_chr == '@' {
        return TokenType::SignAt
    } else if current_chr == '(' {
        return TokenType::ParensLeft
    } else if current_chr == ')' {
        return TokenType::ParensRight
    } else if current_chr == '{' {
        return TokenType::SignBracketLeft
    } else if current_chr == '}' {
        return TokenType::SignBracketRight
    } else if current_chr == '\\' {
        return TokenType::SlashBackward
    } else {
        return TokenType::SignUnk
    }
}
// The current character is a punctuation
pub fn handle_punct(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError> {

    if current_token.clone().is_none() {
        current_token = Some( 
            Token {
                id: match_punct(current_chr.clone()),
                value: None
            });
        return Ok((main_collection, current_token))

    } else {
        
        let current_id: TokenType = current_token.clone().unwrap().id;
    
        match current_id {
            TokenType::SignMinus => {
                if current_chr == '>' {
                    current_token = Some (
                        Token {
                            id: TokenType::AssignRight,
                            value: None
                        });
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                } else {
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some( 
                        Token {
                            id: match_punct(current_chr.clone()),
                            value: None
                        });
                    return Ok((main_collection, current_token))
                }
            },
            TokenType::SignLt => {
                if current_chr == '-' {
                    current_token = Some(
                        Token {
                            id: TokenType::AssignLeft,
                            value: None
                        });
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                } else {
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some( 
                        Token {
                            id: match_punct(current_chr.clone()),
                            value: None
                        });
                    return Ok((main_collection, current_token))
                }
            },
            TokenType::SignPrcnt => {
                if current_chr == '%' {
                    current_token = Some ( 
                        Token {
                            id: TokenType::SignModulo,
                            value: None
                        });
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                } else if current_chr == '>' {
                    current_token = Some (
                        Token {
                            id: TokenType::PipeDplyr,
                            value: None
                        });
                    return Ok((main_collection, current_token))
                } else {
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some( 
                        Token {
                            id: TokenType::SignPrcnt,
                            value: None
                        });
                    return Ok((main_collection, current_token))
                }
            },
            TokenType::PipeDplyr => {
                if current_chr == '%' {
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                } else {
                    return Err( TokeError {
                        id: TokeErrType::UnexpectedSpecialSignInturrupt
                    })
                }
            },
            TokenType::SpIn => {
                if current_chr == '%' {
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                } else {
                    return Err( TokeError {
                        id: TokeErrType::UnexpectedSpecialSignInturrupt
                    })
                }
            },
            TokenType::Char => {
                if( current_chr == '.' ||
                    current_chr == '_'
                ){
                    current_token = concat_value(current_token, current_chr);
                    return Ok((main_collection, current_token))
                } else { 
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some(
                        Token {
                            id: match_punct(current_chr.clone()),
                            value: None
                        });
                    return Ok((main_collection, current_token))
                }
            },
            _ => {
                (main_collection, _) = push_to_main(main_collection, current_token);
                current_token = Some( 
                    Token {
                        id: match_punct(current_chr.clone()),
                        value: None
                    });
                return Ok((main_collection, current_token))
            }
        }
    }
}








