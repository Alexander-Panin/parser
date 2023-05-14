use std::iter::{Peekable};
use crate::atoms::{Token};

const KEYWORDS: [(&str, Token); 6] = [
    ("const", Token::Const), 
    ("function", Token::Function), 
    ("if", Token::If), 
    ("let", Token::Let),
    ("var", Token::Var),
    ("while", Token::While),
];

fn number<I>(fst: &mut Peekable<I>) 
    where 
        I: Iterator<Item=u8> {
    while let Some(&ch) = fst.peek() {
        match ch as char {
            '0'..='9' => { },
            _ => break,
        }
        fst.next();
    }
}

fn variable<I>(fst: &mut Peekable<I>, x: char) -> Token
    where 
        I: Iterator<Item=u8> {
    let mut r = vec![x];
    while let Some(&ch) = fst.peek() {
        match ch as char {
            'A'..='Z' | 'a'..='z' | '0'..='9' => { r.push(ch as char) },
            _ => break,
        }
        fst.next();
    }
    let s: String = r.iter().collect();
    let i = KEYWORDS.partition_point(|x| x.0 < &s);
    let ok = i != KEYWORDS.len() && KEYWORDS[i].0 == s;
    if ok { KEYWORDS[i].1 } else { Token::Variable }
}

pub fn tokens<I>(mut fst: Peekable<I>) -> Vec<Token> 
    where 
        I: Iterator<Item=u8> {
    let mut result = vec![];
    while let Some(ch) = fst.next() {
        match ch as char {
            '0'..='9' => {
                result.push(Token::Number);
                number(&mut fst);
            },
            'A'..='Z' | 'a'..='z' => {
                result.push(variable(&mut fst, ch as char));
            },
            '=' => result.push(Token::EqualSign),
            '-' => result.push(Token::Minus),
            '*' | '+' | '/' => result.push(Token::Operator),
            '(' => result.push(Token::BracketLeft),
            ')' => result.push(Token::BracketRight),
            '{' => result.push(Token::CurlyBracketLeft),
            '}' => result.push(Token::CurlyBracketRight),
            ';' => result.push(Token::Semicolon),
            ',' => result.push(Token::Comma),
            ' ' => {}, // space 
            '\n' => {}, // space 
            _ => { }, // space 
        }
    }
    println!("{:#?}", result);
    result
}


