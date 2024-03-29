#[allow(unused_parens)]
#[allow(non_snake_case)]


use std::fmt;
use std::path::Path;
use std::io::Write;

use crate::space;
use crate::chars;
use crate::num;
use crate::punct;
use crate::log::{ Log, TokenizeStage, get_log_name };



#[derive(Debug)]
pub enum TokeErrType {
    UnexpectedChar,
    UnexpectedSign,
    UnexpectedSpace,
    UnexpectedSpecialCharInturrupt,
    UnexpectedSpecialSignInturrupt,
}
#[derive(Debug)]
pub struct TokeError {
    pub id: TokeErrType
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Start,
    End,

    Space,
    Char,
    Num,

    StringSnglSt,
    StringSnglQt,
    StringDblSt,
    StringDblQt,
    StringComment,

    SignEq,
    SignMinus,
    SignGt,
    SignLt,
    SignPrcnt,
    SignModulo,
    SignPeriod,
    SignComma,
    SignUnderScore,
    SignAt,
    SignBracketRight,
    SignBracketLeft,
    SignExclam,
    SignHash,
    SignDol,
    SignHat,
    SignAmp,
    SignAskt,
    SignSqBracketLeft,
    SignSqBracketRight,
    SignPipe,
    SignTilda,
    SignColon,
    SignPlus,
    SignBackTick,

    SlashForward,
    SlashBackward,
    
    ParensRight,
    ParensLeft,

    SignUnk,

    AssignRight,
    AssignLeft,
    
    PipeDplyr,

    SpIn,
    SpUnk,
}

impl fmt::Display for TokenType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
//            TokenType::Start            => write!(formatter, "Start"),
//            TokenType::End              => write!(formatter, "End"),
//            TokenType::Space            => write!(formatter, "Space"),
//            TokenType::Char             => write!(formatter, "Char"),
//            TokenType::Num              => write!(formatter, "Num"),
//            TokenType::StringSnglSt     => write!(formatter, "String
            TokenType::SignGt           => write!(formatter, "{}", '>'),
            TokenType::SignLt           => write!(formatter, "{}", '<'),
            TokenType::SignPrcnt        => write!(formatter, "{}", '%'),
            TokenType::SignModulo       => write!(formatter, "{}", "%%"),
            TokenType::SignPeriod       => write!(formatter, "{}", '.'),
            TokenType::SignComma        => write!(formatter, "{}", ','),
            TokenType::SignUnderScore   => write!(formatter, "{}", '_'),
            TokenType::SignAt           => write!(formatter, "{}", '@'),
            TokenType::SignBracketRight => write!(formatter, "{}", ']'),
            TokenType::SignBracketLeft  => write!(formatter, "{}", '['),
            TokenType::SignExclam       => write!(formatter, "{}", '!'),
            TokenType::SignHash         => write!(formatter, "{}", '#'),
            TokenType::SignDol          => write!(formatter, "{}", '$'),
            TokenType::SignHat          => write!(formatter, "{}", '^'),
            TokenType::SignAmp          => write!(formatter, "{}", '&'),
            TokenType::SignAskt         => write!(formatter, "{}", '*'),
            TokenType::SignSqBracketLeft=> write!(formatter, "{}", '['),
            TokenType::SignSqBracketRight=>write!(formatter, "{}", ']'),
            TokenType::SignPipe         => write!(formatter, "{}", '|'),
            TokenType::SignTilda        => write!(formatter, "{}", '~'),
            TokenType::SignColon        => write!(formatter, "{}", ':'),
            TokenType::SignPlus         => write!(formatter, "{}", '+'),
            TokenType::SignBackTick     => write!(formatter, "{}", '`'),
            
            _ => write!(formatter, "Unknown type") 
            
            
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub id: TokenType, 
    pub value: Option<Vec<String>>
}

impl Token {
    pub fn start() -> Self {
        return 
            Token {
                id: TokenType::Start,
                value: None
        }
    }

    pub fn end() -> Self {
        return 
            Token {
                id: TokenType::End,
                value: None
        }
    }

    pub fn record_token_in_log(self) {
        let file_path = Path::new("./..")
            .join("target")
            .join("logs");
        let file_list = std::fs::read_dir(file_path);
            match file_list {
                Ok(_) => {
                    let filename_log = get_log_name();
                    let mut log_file = std::fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(filename_log)
                        .expect("Cannot append to log file");
                    let token_text = format!("{:?}\n", self);
                    let _ = log_file.write(token_text.as_bytes());
                },
                Err(_) => println!("Error")
            }
    }
}


pub fn push_to_main(main_collection: Vec<Token>, current_token: Option<Token>) -> (Vec<Token>, Option<Token>) {
    let mut main_collection_new: Vec<Token> = main_collection.clone();
    main_collection_new.push(current_token.unwrap());
    let current_token_new: Option<Token> = None;

    return (main_collection_new, current_token_new)

}

pub fn start_string_sngl(current_chr: char) -> Option<Token> {
    return Some(
        Token {
            id: TokenType::StringSnglQt,
            value: Some(vec![current_chr.to_string()])
        }
    )
}

pub fn start_string_dbl(current_chr: char) -> Option<Token> {
    return Some (
        Token {
            id: TokenType::StringDblQt,
            value: Some(vec![current_chr.to_string()])
        }
    )
}

pub fn concat_value(current_token: Option<Token>, current_chr: char) -> Option<Token> {
    let current_id: TokenType = current_token.clone().unwrap().id;
    let mut current_value = current_token.clone().unwrap().value.unwrap(); 
    current_value.push(current_chr.to_string());
    let current_string = current_value.concat();

    return Some(
        Token {
            id: current_id,
            value: Some(vec![current_string])
        })

}

//// TOKENIZE

pub fn tokenize(code: &str) -> Result<Vec<Token>, TokeError> {
    Log::location(TokenizeStage::Start).write();
    let mut main_collection = vec![Token::start()];
    let mut current_token: Option<Token> = None;
    let code_buffer = code;
    let code_length = code_buffer.len() - 1;

    for current_buffer in code_buffer.chars().enumerate(){
        let current_indx = current_buffer.0;
        let current_chr = current_buffer.1;
        println!("Current indx: {:?}", current_indx);
        println!("Current chr: {:?}", current_chr);

        if space::is_space(current_chr.clone()) {
            println!("Is space");
            let space_response = space::handle_space(main_collection.clone(), current_token.clone(), current_chr);
            match space_response {
                Ok((mc, ct)) => {
                    main_collection = mc;
                    current_token = ct;
                },
                Err(e) => {
                    println!("Error handling space {:?}", e);
                }
            }
        } else if punct::is_punct(current_chr.clone()) {
            println!("Is punct");
            let punct_response = punct::handle_punct(main_collection.clone(), current_token.clone(), current_chr);
            match punct_response {
                Ok((mc, ct)) => {
                    main_collection = mc;
                    current_token = ct;
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        } else if chars::is_char(current_chr.clone()) {
            println!("Is char");
            let chr_response = chars::handle_char(main_collection.clone(), current_token.clone(), current_chr);
            match chr_response {
                Ok((mc, ct)) => {
                    main_collection = mc;
                    current_token = ct;
                },
                Err(e) =>  {
                    println!("{:?}", e);
                }
             }
        } else if num::is_num(current_chr.clone()) {
            println!("Is num");
            let num_response = num::handle_num(main_collection.clone(), current_token.clone(), current_chr);
            match num_response {
                Ok((mc, ct)) => {
                    main_collection = mc;
                    current_token = ct;
                }, 
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }

        if current_token.clone().is_some() {
            (main_collection, _) = push_to_main(main_collection.clone(), current_token.clone());
        }
        (main_collection, _) = push_to_main(
            main_collection.clone(),
            Some(Token::end()));
        
    Ok(main_collection)
}
