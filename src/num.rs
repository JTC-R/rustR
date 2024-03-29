#[allow(unused_parens)]
#[allow(non_snake_case)]

use crate::tokenize:: {Token, TokenType, start_string_sngl, start_string_dbl, concat_value, push_to_main};
use crate::tokenize::TokeError;
use crate::log::{TokenizeStage, Log};


pub fn is_num(current_chr: char) -> bool {
    if current_chr.is_numeric() {
        return true 
    } else {
        return false
    }
}

pub fn handle_num(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError>{
    Log::location(TokenizeStage::Num).write();
    if current_token.clone().is_none() {
        current_token = Some( 
            Token { 
                id: TokenType::Num,
                value: Some(vec![current_chr.to_string()])
            });
        return Ok((main_collection, current_token))
    } else {
        let current_type: TokenType = current_token.clone().unwrap().id;
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
            TokenType::StringComment => {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::Num => {
                current_token = Some( 
                    Token {
                        id: TokenType::Num,
                        value: Some(vec![current_chr.to_string()])
                    }
                );
                return Ok((main_collection, current_token))
            },
            TokenType::Char => {
                current_token = concat_value(current_token, current_chr.clone());
                return Ok((main_collection, current_token))
            },
            _ => {
                (main_collection, _) = push_to_main(main_collection, current_token);
                current_token = Some( 
                    Token {
                        id: TokenType::Num,
                        value: Some(vec![current_chr.to_string()])
                    });
                return Ok((main_collection, current_token))
            }
        }
    }
}
