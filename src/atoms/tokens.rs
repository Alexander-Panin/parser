use std::iter::{Peekable};
use crate::atoms::{Token};

const KEYWORDS: [(&str, Token); 14] = [
    ("await", Token::Await),
    ("class", Token::Class),
    ("const", Token::Const),
    ("extends", Token::Extends),
    ("false", Token::False),
    ("function", Token::Function),
    ("if", Token::If),
    ("let", Token::Let),
    ("new", Token::New),
    ("null", Token::Null),
    ("return", Token::Return),
    ("true", Token::True),
    ("var", Token::Var),
    ("while", Token::While),
];

// todo: not fully covered operator topic 
fn operator<I>(fst: &mut Peekable<I>)
    where
        I: Iterator<Item=u8> {
    if let Some(&ch) = fst.peek() {
        match ch as char {
            '*' | '&' | '|' => { fst.next(); },
            _ => { },
        }
    }
}

// todo fix plz real numbers
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

fn dots<I>(fst: &mut Peekable<I>) -> Token
    where
        I: Iterator<Item=u8> {
    let mut k = 0;
    for _ in 0..2 {
        if let Some(&ch) = fst.peek() {
            match ch as char {
                '.' =>  { k += 1; },
                _ => break,
            }
            fst.next();
        }
    }
    [Token::Dot, Token::Dot2, Token::Dot3][k]
}

// todo need to fix for quoting 
fn string<I>(fst: &mut Peekable<I>, x: char)
    where
        I: Iterator<Item=u8> {
    for ch in fst.by_ref() {
        match ch as char {
            q if q == x => break,
            _ => { },
        }
    }
}

// todo add plz match by underscore
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


fn operator_equal<I>(fst: &mut Peekable<I>) -> Token
    where
        I: Iterator<Item=u8> {
    if let Some(&ch) = fst.peek() {
        match ch as char {
            '=' => {
                fst.next();
                if fst.peek() == Some(&b'=') {
                    fst.next();
                }
                return Token::Operator;
            },
            '>' => {
                fst.next();
                return Token::FatArrow;
            },
            _ => { },
        }
    }
    Token::EqualSign
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
            '=' => {
                result.push(operator_equal(&mut fst));
            },
            '-' => result.push(Token::Minus),
            '*' | '+' | '/' | '&' | '|' => {
                result.push(Token::Operator);
                operator(&mut fst);
            },
            '"' | '\'' => {
                result.push(Token::String);
                string(&mut fst, ch as char);
            },
            '(' => result.push(Token::BracketLeft),
            ')' => result.push(Token::BracketRight),
            '{' => result.push(Token::CurlyBracketLeft),
            '}' => result.push(Token::CurlyBracketRight),
            '[' => result.push(Token::SquareBracketLeft),
            ']' => result.push(Token::SquareBracketRight),
            ';' => result.push(Token::Semicolon),
            ':' => result.push(Token::Colon),
            ',' => result.push(Token::Comma),
            '.' => {
                result.push(dots(&mut fst));
            },
            ' ' => {}, // space
            '\n' => {}, // space
            _ => { }, // space
        }
    }
    // println!("{:#?}", result);
    result
}


