use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}



impl <'a> Tokenizer<'a>{
    pub fn new(new_expr: &'a str)-> Self {
        Tokenizer{
            expr: new_expr.chars().peekable(),
        }
    }
}

impl <'a> Iterator for Tokenizer<'a>{
    type Item = Token;

    fn next(&mut self)-> Option<Token>{
        let next_char = self.expr.next();
        match next_char {
            Some('0'..='9') => {
                let mut number =  next_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?) 
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
            Some(Token::Num(number.parse::<f64>().unwrap()))
            },
            Some('a') => Some(Token::Add),
            Some('b') => Some(Token::Subtract),
            Some('c') => Some(Token::Multiply),
            Some('d') => Some(Token::Divide),
            Some('e') => Some(Token::LeftParen),
            Some('f') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

