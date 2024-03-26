#[allow(unused_parens)]

pub mod tokenize;
pub mod space;
pub mod chars;
pub mod num;
pub mod punct;
pub mod errorHandler;
pub mod log;
pub mod init;


fn main() {
    init::init();
    let input_code = "a";
       // tryCatch(
       //         expr = {
       //             test_foo() -> bar
       //             if 'test' %in% bar[5, c('col_one', 'col_two')]
       //         }, error = function(e){
       //             print(e)
       //         })
       //     ";
    let token = tokenize::tokenize(input_code).unwrap();
        
    println!("Input code: {:?}", input_code);
    println!("{:?}", token);
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

}



