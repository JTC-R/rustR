#[allow(unused_parens)]
#[allow(non_snake_case)]

use std::env;
use std::fs::{File, OpenOptions};
use std::io::Read;

pub mod tokenize;
pub mod space;
pub mod chars;
pub mod num;
pub mod punct;
pub mod errorHandler;
pub mod log;
pub mod init;

use crate::log::*;

fn main() {
    init::init();
    
    let mut file_location = &String::new();
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
       panic!("No file path supplied");
    } else {
        file_location = &args[1];
        Log::record(
            Some(LogType::Notify),
            Some(TokenizeStage::Init),
            Some(TokenizeAction::InitArgsSupplied)
        );
    }

    let mut file_con = OpenOptions::new()
        .read(true)
        .open(&file_location)
        .expect("Cannot locate given file");

    let mut input_code = String::new();
    file_con.read_to_string(&mut input_code)
        .expect("Cannot read file");

    println!("{:?}", input_code);


    let tokens = tokenize::tokenize(&input_code).unwrap();
        
    println!("Input code: {:?}\n", input_code);
    //println!("{:?}\n", tokens);

    for token in tokens.into_iter() {
//        println!("Placing token in log\n");
//        println!("{:?}", tok.clone());
        token.record_token_in_log(); 
    }


}

#[cfg(test)]
mod test {
    #[allow(unused_parens)]
    use super::*;
    use crate::tokenize::tokenize;
    use crate::tokenize::{Token, TokenType};
    use crate::tokenize::{TokeError, TokeErrType};
    use crate::init;
    use crate::log::*;

    #[test]
    fn unit_char() {
        let tokens = tokenize::tokenize("abc");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["abc".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::End,
                value: None
            });
    }

    #[test]
    fn unit_nums() {
        let tokens = tokenize::tokenize("123");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Num,
                value: Some(vec!["123".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::End,
                value: None
            });
    }
    
    #[test]
    fn char_space_num() {
        let tokens = tokenize::tokenize("abc 123");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["abc".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::Num,
                value: Some(vec!["123".to_string()])
            });
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::End,
                value: None
            });
    }

    #[test]
    fn assign_num_to_char() {
        let tokens = tokenize::tokenize("abc = 123");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        
        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["abc".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::Num,
                value: Some(vec!["123".to_string()])
            });
        assert_eq!(
            tokens[4],
            Token {
                id: TokenType::End,
                value: None
            });
    }
    
    #[test]
    fn num_assignRight_char() {
        let tokens = tokenize::tokenize("098 -> zyv");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Num,
                value: Some(vec!["098".to_string()])
            });
        assert_eq!(
            tokens[2], 
            Token {
                id: TokenType::AssignRight,
                value: None
            });
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::Char,
                value: Some(vec!["zyv".to_string()])
            });
        assert_eq!(  
            tokens[4],
            Token {
                id: TokenType::End,
                value: None
            });
        
    }

    #[test]
    fn char_assignLeft_num() {
        let tokens = tokenize::tokenize("jfv <- 532");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        
        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["jfv".to_string()]) 
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::AssignLeft,
                value: None
            }); 
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::Num,
                value: Some(vec!["532".to_string()])
            });
        assert_eq!(
            tokens[4],
            Token {
                id: TokenType::End,
                value: None
            });
    }
    
    #[test]
    fn charAtChar_assignRight_char() {
        let tokens = tokenize::tokenize("test@example -> fa2");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        
        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["test".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::SignAt,
                value: None
            });
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::Char,
                value: Some(vec!["example".to_string()])
            });
        assert_eq!(
            tokens[4],
            Token {
                id: TokenType::AssignRight,
                value: None
            });
        assert_eq!(
            tokens[5],
            Token {
                id: TokenType::Char,
                value: Some(vec!["fa2".to_string()])
            });
        assert_eq!(
            tokens[6],
            Token {
                id: TokenType::End,
                value: None
            });
    }

    #[test]
    fn charParensWithNumber_assignRight_char() {
        let tokens = tokenize::tokenize("test_func(123) -> abc");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(
            tokens[0],
            Token { 
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["test_func".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::Num,
                value: Some(vec!["123".to_string()])
            });
        assert_eq!(
            tokens[4],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[5],
            Token {
                id: TokenType::AssignRight,
                value: None
            });
        assert_eq!(
            tokens[6],
            Token {
                id: TokenType::Char,
                value: Some(vec!["abc".to_string()])
            });
        assert_eq!(
            tokens[7],
            Token {
                id: TokenType::End,
                value: None
            });
    }

    #[test]
    fn testBrackets_and_Parens() {
        let tokens = tokenize::tokenize("
            tryCatch(
                expr = {
                    test_foo() -> bar
                }, error = function(e){
                    print(e)
                })
            ");
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["tryCatch".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::Char,
                value: Some(vec!["expr".to_string()])
            });
        assert_eq!(
            tokens[4],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[5],
            Token {
                id: TokenType::SignBracketLeft,
                value: None
            });
        assert_eq!(
            tokens[6],
            Token {
                id: TokenType::Char,
                value: Some(vec!["test_foo".to_string()])
            });
        assert_eq!(
            tokens[7],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[8],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[9],
            Token {
                id: TokenType::AssignRight,
                value: None
            });
        assert_eq!(
            tokens[10],
            Token {
                id: TokenType::Char,
                value: Some(vec!["bar".to_string()])
            });
        assert_eq!(
            tokens[11],
            Token {
                id: TokenType::SignBracketRight,
                value: None
            });
        assert_eq!(
            tokens[12],
            Token {
                id: TokenType::SignComma,
                value: None
            });
        assert_eq!(
            tokens[13],
            Token {
                id: TokenType::Char,
                value: Some(vec!["error".to_string()])
            });
        assert_eq!(
            tokens[14],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[15],
            Token {
                id: TokenType::Char,
                value: Some(vec!["function".to_string()])
            });
        assert_eq!(
            tokens[16],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[17],
            Token {
                id: TokenType::Char,
                value: Some(vec!['e'.to_string()])
            });
        assert_eq!(
            tokens[18],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[19],
            Token {
                id: TokenType::SignBracketLeft,
                value: None
            });
        assert_eq!(
            tokens[20],
            Token {
                id: TokenType::Char,
                value: Some(vec!["print".to_string()])
            });
        assert_eq!(
            tokens[21],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[22],
            Token {
                id: TokenType::Char,
                value: Some(vec!['e'.to_string()])
            });
        assert_eq!(
            tokens[23],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[24],
            Token {
                id: TokenType::SignBracketRight,
                value: None
            });
        assert_eq!(
            tokens[25],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[26],
            Token {
                id: TokenType::End,
                value: None
            });
    }

    #[test]
    fn function_declaration_med_complex() {

        let tokens = tokenize::tokenize("

            main_function <- function(inputDate.user_date, inputStr.user_name) {
                format_str(inputStr.user_name) -> str_usr_name
                user_name = inputStr.user_name

                data <- data.frame('a' = c(str_usr_name),
                           'b' = c(user_name))

                print(paste0(data$col_name, '_data!@#$)')

                data[5:15, c('b')] %>%
                    mutate(t = lm(. ~ a)) %>%
                    filter(aj53 %in% t) -> data

                data %>%
                    ggplot() + geom_line(aes(x = `test`, y = la))
            }

            ");
        
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(
            tokens[0],
            Token {
                id: TokenType::Start,
                value: None
            });
        assert_eq!(
            tokens[1],
            Token {
                id: TokenType::Char,
                value: Some(vec!["main_function".to_string()])
            });
        assert_eq!(
            tokens[2],
            Token {
                id: TokenType::AssignLeft,
                value: None
            });
        assert_eq!(
            tokens[3],
            Token {
                id: TokenType::Char,
                value: Some(vec!["function".to_string()])
            });
        assert_eq!(
            tokens[4],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[5],
            Token {
                id: TokenType::Char,
                value: Some(vec!["inputDate.user_date".to_string()])
            });
        assert_eq!(
            tokens[6],
            Token {
                id: TokenType:: SignComma,
                value: None
            });
        assert_eq!(
            tokens[7],
            Token {
                id: TokenType::Char,
                value: Some(vec!["inputStr.user_name".to_string()])
            });
        assert_eq!(
            tokens[8],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[9],
            Token {
                id: TokenType::SignBracketLeft,
                value: None
            });
        assert_eq!(
            tokens[10],
            Token {
                id: TokenType::Char,
                value: Some(vec!["format_str".to_string()])
            });
        assert_eq!(
            tokens[11],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[12],
            Token {
                id: TokenType::Char,
                value: Some(vec!["inputStr.user_name".to_string()])
            });
        assert_eq!(
            tokens[13],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[14],
            Token {
                id: TokenType::AssignRight,
                value: None
            });
        assert_eq!(
            tokens[15],
            Token {
                id: TokenType::Char,
                value: Some(vec!["str_usr_name".to_string()])
            });
        assert_eq!(
            tokens[16],
            Token {
                id: TokenType::Char,
                value: Some(vec!["user_name".to_string()])
            });
        assert_eq!(
            tokens[17],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[18],
            Token {
                id: TokenType::Char,
                value: Some(vec!["inputStr.user_name".to_string()])
            });
        assert_eq!(
            tokens[19],
            Token {
                id: TokenType::Char,
                value: Some(vec!["data".to_string()])
            });
        assert_eq!(
            tokens[20],
            Token {
                id: TokenType::AssignLeft,
                value: None
            });
        assert_eq!(
            tokens[21],
            Token {
                id: TokenType::Char,
                value: Some(vec!["data.frame".to_string()])
            });
        assert_eq!(
            tokens[22],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[23],
            Token {
                id: TokenType::StringSnglQt,
                value: Some(vec!['a'.to_string()])
            });
      assert_eq!(
          tokens[24],
          Token {
              id: TokenType::SignEq,
              value: None
          });
      assert_eq!(
          tokens[25],
          Token {
              id: TokenType::Char,
              value: Some(vec!['c'.to_string()])
          });
        assert_eq!(
            tokens[26],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[27],
            Token {
                id: TokenType::Char,
                value: Some(vec!["str_usr_name".to_string()])
            });
        assert_eq!(
            tokens[28],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[29],
            Token {
                id: TokenType::SignComma,
                value: None
            });
        assert_eq!(
            tokens[30],
            Token {
                id: TokenType::StringSnglQt,
                value: Some(vec!['b'.to_string()])
            });
        assert_eq!(
            tokens[31],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[32],
            Token {
                id: TokenType::Char,
                value: Some(vec!['c'.to_string()])
            });
        assert_eq!(
            tokens[33],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[34],
            Token {
                id: TokenType::Char,
                value: Some(vec!["user_name".to_string()])
            });
        assert_eq!(
            tokens[35],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[36],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[37],
            Token {
                id: TokenType::Char,
                value: Some(vec!["print".to_string()])
            });
        assert_eq!(
            tokens[38],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[39],
            Token {
                id: TokenType::Char,
                value: Some(vec!["paste0".to_string()])
            });
        assert_eq!(
            tokens[40],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[41],
            Token {
                id: TokenType::Char,
                value: Some(vec!["data".to_string()])
            });
        assert_eq!(
            tokens[42],
            Token {
                id: TokenType::SignDol,
                value: None
            });
        assert_eq!(
            tokens[43],
            Token {
                id: TokenType::Char,
                value: Some(vec!["col_name".to_string()])
            });
        assert_eq!(
            tokens[44],
            Token {
                id: TokenType::SignComma,
                value: None
            });
        assert_eq!(
            tokens[45],
            Token {
                id: TokenType::StringSnglQt,
                value: Some(vec!["_data!@#$)".to_string()])
            });
        assert_eq!(
            tokens[46],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[47],
            Token {
                id: TokenType::Char,
                value: Some(vec!["data".to_string()])
            });
        assert_eq!(
            tokens[48],
            Token {
                id: TokenType::SignSqBracketLeft,
                value: None
            });
        assert_eq!(
            tokens[49],
            Token {
                id: TokenType::Num,
                value: Some(vec!['5'.to_string()])
            });
        assert_eq!(
            tokens[50],
            Token {
                id: TokenType::SignColon,
                value: None
            });
        assert_eq!(
            tokens[51],
            Token {
                id: TokenType::Num,
                value: Some(vec!["15".to_string()])
            });
        assert_eq!(
            tokens[52],
            Token {
                id: TokenType::SignComma,
                value: None
            });
        assert_eq!(
            tokens[53],
            Token {
                id: TokenType::Char,
                value: Some(vec!['c'.to_string()])
            });
        assert_eq!(
            tokens[54],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[55],
            Token {
                id: TokenType::StringSnglQt,
                value: Some(vec!['b'.to_string()])
            });
        assert_eq!(
            tokens[56],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[57],
            Token {
                id: TokenType::SignSqBracketRight,
                value: None
            });
        assert_eq!(
            tokens[58],
            Token {
                id: TokenType::PipeDplyr,
                value: None
            });
        assert_eq!(
            tokens[59],
            Token {
                id: TokenType::Char,
                value: Some(vec!["mutate".to_string()])
            });
        assert_eq!(
            tokens[60],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[61],
            Token {
                id: TokenType::Char,
                value: Some(vec!['t'.to_string()])
            });
        assert_eq!(
            tokens[62],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[63],
            Token {
                id: TokenType::Char,
                value: Some(vec!["lm".to_string()])
            });
        assert_eq!(
            tokens[64],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[65],
            Token {
                id: TokenType::SignPeriod,
                value: None
            });
        assert_eq!(
            tokens[66],
            Token {
                id: TokenType::SignTilda,
                value: None
            });
        assert_eq!(
            tokens[67],
            Token {
                id: TokenType::Char,
                value: Some(vec!['a'.to_string()])
            });
        assert_eq!(
            tokens[68],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[69],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[70],
            Token {
                id: TokenType::PipeDplyr,
                value: None
            });
        assert_eq!(
            tokens[71],
            Token {
                id: TokenType::Char,
                value: Some(vec!["filter".to_string()])
            });
        assert_eq!(
            tokens[72],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[73],
            Token {
                id: TokenType::Char,
                value: Some(vec!["aj53".to_string()])
            });
        assert_eq!(
            tokens[74],
            Token {
                id: TokenType::SpIn,
                value: None
            });
        assert_eq!(
            tokens[75],
            Token {
                id: TokenType::Char,
                value: Some(vec!['t'.to_string()])
            });
        assert_eq!(
            tokens[76],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[77],
            Token {
                id: TokenType::AssignRight,
                value: None
            });
        assert_eq!(
            tokens[78],
            Token {
                id: TokenType::Char,
                value: Some(vec!["data".to_string()])
            });
        assert_eq!(
            tokens[79],
            Token {
                id: TokenType::Char,
                value: Some(vec!["data".to_string()])
            });
        assert_eq!(
            tokens[80],
            Token {
                id: TokenType::PipeDplyr,
                value: None
            });
        assert_eq!(
            tokens[81],
            Token {
                id: TokenType::Char,
                value: Some(vec!["ggplot".to_string()])
            });
        assert_eq!(
            tokens[82],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[83],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[84],
            Token {
                id: TokenType::SignPlus,
                value: None
            });
        assert_eq!(
            tokens[85],
            Token {
                id: TokenType::Char,
                value: Some(vec!["geom_line".to_string()])
            });
        assert_eq!(
            tokens[86],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[87],
            Token {
                id: TokenType::Char,
                value: Some(vec!["aes".to_string()])
            });
        assert_eq!(
            tokens[88],
            Token {
                id: TokenType::ParensLeft,
                value: None
            });
        assert_eq!(
            tokens[89],
            Token {
                id: TokenType::Char,
                value: Some(vec!['x'.to_string()])
            });
        assert_eq!(
            tokens[90],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[91],
            Token {
                id: TokenType::SignBackTick,
                value: None
            });
        assert_eq!(
            tokens[92],
            Token {
                id: TokenType::Char,
                value: Some(vec!["test".to_string()])
            });
        assert_eq!(
            tokens[93],
            Token {
                id: TokenType::SignBackTick,
                value: None
            });
        assert_eq!(
            tokens[94],
            Token {
                id: TokenType::SignComma,
                value: None
            });
        assert_eq!(
            tokens[95],
            Token {
                id: TokenType::Char,
                value: Some(vec!['y'.to_string()])
            });
        assert_eq!(
            tokens[96],
            Token {
                id: TokenType::SignEq,
                value: None
            });
        assert_eq!(
            tokens[97],
            Token {
                id: TokenType::Char,
                value: Some(vec!["la".to_string()])
            });
        assert_eq!(
            tokens[98],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[99],
            Token {
                id: TokenType::ParensRight,
                value: None
            });
        assert_eq!(
            tokens[100],
            Token {
                id: TokenType::SignBracketRight,
                value: None
            });
        assert_eq!(
            tokens[101],
            Token {
                id: TokenType::End,
                value: None
            });

    }
}
