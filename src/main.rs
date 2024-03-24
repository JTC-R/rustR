#[allow(unused_parens)]

pub mod tokenize;
pub mod space;
pub mod chars;
pub mod num;
pub mod punct;


fn main() {
    let input_code = "ab_c. -> 123";

    let token = tokenize::tokenize(input_code).unwrap();
    
    println!("Input code: {:?}", input_code);
    println!("{:?}", token);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokenize::tokenize;
    use crate::tokenize::{Token, TokenType};
    use crate::tokenize::{TokeError, TokeErrType};

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

}





























