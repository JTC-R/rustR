#[allow(unused_parens)]
use std::thread::current;
use crate::tokenize:: {Token, TokenType, start_string_single, start_dbl_string, concat_value, push_to_main};
use crate::tokenize::{TokeError, TokeErrType};
use crate::punct;
pub fn is_num(current_chr: char) -> bool {
    if current_chr.is_numeric() {
        return true 
    } else {
        return false
    }
}

pub fn handle_num(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError>{
    if current_token.clone().is_none() {
        current_token = Some( 
            Token { 
                id: TokenType::Num,
                value: Some(vec![current_chr.to_string()])
            });
        return Ok((main_collection, current_token))
    } else {
        let current_type: TokenType = current_token.clone().unwrap().id;
        let current_value_test: Option<Vec<String>> = current_token.clone().unwrap().value;
        let mut current_value_unwrapped = Vec::new();

        if current_value_test.is_some() {
            current_value_unwrapped = current_token.clone().unwrap().value.unwrap();
            current_value_unwrapped.push(current_chr.to_string());
            current_value_unwrapped = vec![current_value_unwrapped.concat()];
        }
        
        match current_type {
            TokenType::StringSngSt => {
                current_token = start_string_single(current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringSnglQt => {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringDblSt => {
                current_token = start_dbl_string(current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringDblQt => {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            }
            TokenType::Num => {
                current_token = Some( 
                    Token {
                        id: TokenType::Num,
                        value: Some(current_value_unwrapped)
                    }
                );
                return Ok((main_collection, current_token))
            },
            TokenType::Char => {
                current_token = concat_value(current_token, current_chr.clone());
                return Ok((main_collection, current_token))
            },
            _ => {
                if punct::is_punct(current_chr.clone()) {
                    (main_collection, _) = push_to_main(main_collection, current_token);
                    current_token = Some( Token {
                        id: punct::match_punct(current_chr.clone()),
                        value: None
                    });
                    return Ok((main_collection, current_token))
                } else {
                    current_token = Some( 
                        Token {
                            id: TokenType::Num,
                            value: Some(current_value_unwrapped)
                        }
                    );
                    return Ok((main_collection, current_token))
                }
            }
        }
    }
}
