#[allow(unused_parens)]
#[allow(non_snake_case)]
use std::thread::current;
use crate::tokenize:: {Token, TokenType, start_string_sngl, start_string_dbl, push_to_main, concat_value};
use crate::tokenize:: {TokeError, TokeErrType};
use crate::log::{ Log, LogType, TokenizeStage, TokenizeAction };

pub fn is_space(current_chr: char) -> bool {
    if (
        current_chr.clone().to_string() == ' '.to_owned().to_string() ||
        current_chr.clone().to_string() == "\n".to_string()
    ){
        return true
    } else {
        return false
    }
}

pub fn handle_space(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError> {
    Log::location(TokenizeStage::Space).write();
    
    if current_token.clone().is_none() {
       return Ok((main_collection, current_token))
    } else {
        let current_type: TokenType = current_token.clone().unwrap().id;
        if current_type == TokenType::StringComment {
            if current_chr.to_string() == "\n".to_string() {
                (main_collection, current_token) = push_to_main(main_collection, current_token);
                return Ok((main_collection, current_token))
            } else {
                current_token = concat_value(current_token, current_chr);
                return Ok((main_collection, current_token))
            }
        } else {

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
                _ => {
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                }
            }
        }
    }
}
