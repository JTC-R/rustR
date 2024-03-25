use crate::tokenize::{Token, TokenType, start_string_single, start_dbl_string, concat_value, push_to_main};
use crate::tokenize::{TokeError, TokeErrType};
use crate::punct;


pub fn is_char(current_chr: char) -> bool {
    if current_chr.is_alphabetic() {
        return true
    } else {
        return false
    }
}

pub fn handle_char(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError> {

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
            TokenType::StringSngSt => {
                current_token = start_string_single(current_chr);
                return Ok((main_collection, current_token))
            }, 
            TokenType::StringSnglQt => {
                main_collection.push(
                    Token {
                        id: current_type,
                        value: current_value
                    }
                );
                current_token = None;
                return Ok((main_collection, current_token))
            },
            TokenType::StringDblSt => {
                current_token = start_dbl_string(current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringDblQt => {
                main_collection.push(
                    Token {
                        id: current_type,
                        value: current_value
                    }
                );
                current_token = None;
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
            _ => {
                if punct::is_punct(current_chr.clone()) {
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some(
                        Token {
                            id: punct::match_punct(current_chr.clone()),
                            value: None
                        });
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
