use std::thread::current;
use crate::tokenize:: {Token, TokenType, start_string_single, start_dbl_string, push_to_main, concat_value};
use crate::tokenize:: {TokeError, TokeErrType};

pub fn is_space(current_chr: char) -> bool {
    if current_chr.to_string() == ' '.to_owned().to_string() {
        return true
    } else {
        return false
    }
}

pub fn handle_space(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError> {
    if current_token.clone().is_none() {
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
        println!("Inside space");
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
            },
                       // Here is the problem; need to push char / num if not is_none() then set 
            // current to None
            _ => {
                (main_collection, current_token) = push_to_main(main_collection, current_token);
                return Ok((main_collection, current_token))
            }
        }
    }
}
