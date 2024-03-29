
#[allow(non_snake_case)]
#[allow(unused_parens)]
use crate::tokenize::{Token, TokenType, start_string_sngl, start_string_dbl, concat_value, push_to_main};
use crate::tokenize::{TokeError, TokeErrType};
use crate::punct;
use crate::log::{ Log, LogType, TokenizeStage, TokenizeAction };

pub fn is_char(current_chr: char) -> bool {
    if current_chr.is_alphabetic() {
        return true
    } else {
        return false 
    }
}

pub fn handle_char(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError> {

    Log::location(TokenizeStage::Char).write();
    if current_token.clone().is_none() {
        println!("Handle Char - Token - Is None-");
        current_token = Some ( Token {
            id: TokenType::Char,
            value: Some(vec![current_chr.to_string()])
        });
        return Ok((main_collection, current_token))
    } else {

        let current_type: TokenType = current_token.clone().unwrap().id;
        let current_value_test: Option<Vec<String>> = current_token.clone().unwrap().value;
        let mut current_value: Option<Vec<String>> = Some(vec![String::new()]);

        match current_value_test {
            Some(s) => {
                current_value = Some(vec![s.concat()])
            }, 
            None => {
                current_value = None
            }
        }

        match current_type {
            TokenType::StringSnglSt => {
                current_token = start_string_sngl(current_chr);
                return Ok((main_collection, current_token))
            }, 
            TokenType::StringSnglQt => { 
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringDblSt => {
                current_token = start_string_dbl(current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringDblQt => {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::Char => {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::Num => {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::SignPrcnt => {
                if current_chr == 'i' {
                    current_token = Some (
                        Token {
                            id: TokenType::SpIn,
                            value: None
                        });
                    return Ok((main_collection, current_token))
                } else {
                    return Err(TokeError {
                        id: TokeErrType::UnexpectedSpecialCharInturrupt
                    })
                }
            },
            TokenType::SpIn => {
                if current_chr == 'n' {
                   return Ok((main_collection, current_token))
                } else {
                    return Err(TokeError {
                        id: TokeErrType::UnexpectedSpecialCharInturrupt
                    })
                }
            },
            TokenType::SlashBackward => {
                if current_chr == 'n' {
                    current_token = None;
                    return Ok((main_collection, current_token))
                } else {
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some (
                        Token {
                            id: TokenType::Char,
                            value: Some(vec![current_chr.to_string()])
                        });
                    return Ok((main_collection, current_token))
                }
            },
            TokenType::StringComment => {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            }, 
            _ => {
                if (
                    current_type.clone() == TokenType::SignPeriod ||
                    current_type.clone() == TokenType::SignUnderScore
                ){
                    let current_punct = current_type.clone().to_string();
                    current_token = Some(
                        Token {
                            id: TokenType::Char,
                            value: Some(vec![current_punct])
                        });
                    current_token = concat_value(current_token, current_chr);
                    return Ok((main_collection, current_token))
                } else {
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some(
                        Token {
                            id: TokenType::Char,
                            value: Some(vec![current_chr.to_string()])
                        });
                    return Ok((main_collection, current_token))
                }
            }
        }
    }
}
