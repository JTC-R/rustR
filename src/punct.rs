#[allow(unused_parens)]
#[allow(non_snake_case)]

use std::thread::current;
use crate::tokenize::{Token, TokenType, start_string_sngl, start_string_dbl, concat_value, push_to_main};
use crate::tokenize::{TokeError, TokeErrType};
use crate::log::{ Log, LogType, TokenizeStage, TokenizeAction };



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
        current_chr == '\\' ||
        current_chr == '/'  ||
        current_chr == '\'' ||
        current_chr == '"'  ||
        current_chr == '!'  ||
        current_chr == '#'  ||
        current_chr == '$'  ||
        current_chr == '^'  ||
        current_chr == '&'  ||
        current_chr == '*'  ||
        current_chr == '['  ||
        current_chr == ']'  ||
        current_chr == '|'  ||
        current_chr == '~'  ||
        current_chr == ':'  ||
        current_chr == '+'  ||
        current_chr == '`'  
    ) {
        return true
    } else {
        return false
    }
}

pub fn match_punct(current_chr: char) -> TokenType {
    Log::record(
        Some(LogType::Routine),
        Some(TokenizeStage::Punct),
        Some(TokenizeAction::PunctTranslate)
        )
        .write();
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
    } else if current_chr == ':' {
        return TokenType::SignColon
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
    } else if current_chr == '/' {
        return TokenType::SlashForward
    } else if current_chr == '\'' {
        return TokenType::StringSnglSt
    } else if current_chr == '"' {
        return TokenType::StringDblSt
    } else if current_chr == '!' {
        return TokenType::SignExclam
    } else if current_chr == '#' {
        return TokenType::SignHash
    } else if current_chr == '$' {
        return TokenType::SignDol 
    } else if current_chr == '^' {
        return TokenType::SignHat
    } else if current_chr == '&' {
        return TokenType::SignAmp
    } else if current_chr == '*' {
        return TokenType::SignAskt
    } else if current_chr == '[' {
        return TokenType::SignSqBracketLeft
    } else if current_chr == ']' {
        return TokenType::SignSqBracketRight
    } else if current_chr == '|' {
        return TokenType::SignPipe
    } else if current_chr == '~' {
        return TokenType::SignTilda
    } else if current_chr == '+' {
        return TokenType::SignPlus
    } else if current_chr == '`' {
        return TokenType::SignBackTick
    } else {
        return TokenType::SignUnk
    }
}
// The current character is a punctuation
pub fn handle_punct(mut main_collection: Vec<Token>, mut current_token: Option<Token>, current_chr: char) -> Result<(Vec<Token>, Option<Token>), TokeError> {

    Log::location(TokenizeStage::Punct).write();
    if current_token.clone().is_none() {
        if current_chr == '#' {
            current_token = Some( Token {
                id: TokenType::StringComment,
                value: Some(vec![String::new()])
                });
        } else {
            current_token = Some( 
                Token {
                    id: match_punct(current_chr.clone()),
                    value: None
                });
        }
        return Ok((main_collection, current_token))
    } else {
        
        let current_id: TokenType = current_token.clone().unwrap().id;
    
        match current_id {
            TokenType::StringSnglSt => {
                current_token = start_string_sngl(current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringSnglQt => {
                if current_chr == '\'' {
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                } else {
                    Log::location(TokenizeStage::PunctStringSngl).write();
                    current_token = concat_value(current_token, current_chr);
                    return Ok((main_collection, current_token))
                }
            },
            TokenType::StringDblSt => {
                current_token = start_string_dbl(current_chr);
                return Ok((main_collection, current_token))
            },
            TokenType::StringDblQt => {
                if current_chr == '"' {
                    (main_collection, current_token) = push_to_main(main_collection, current_token);
                    return Ok((main_collection, current_token))
                } else {
                    Log::location(TokenizeStage::PunctStringDbl).write();
                    current_token = concat_value(current_token, current_chr);
                    return Ok((main_collection, current_token))
                }
            },
            TokenType::SignHash => {
                (main_collection, _) = push_to_main(main_collection, current_token);
                current_token = Some( 
                    Token {
                        id: TokenType::StringComment,
                        value: Some(vec![String::new()])
                    });
                return Ok((main_collection, current_token))
            }
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
                if( 
                    current_chr == '.' ||
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








