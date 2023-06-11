use crate::atoms::Token;
use std::iter::Peekable;

const KEYWORDS: [(&str, Token); 23] = [
    ("as", Token::As),
    ("await", Token::Await),
    ("class", Token::Class),
    ("const", Token::Const),
    ("default", Token::Default),
    ("export", Token::Export),
    ("extends", Token::Extends),
    ("false", Token::False),
    ("from", Token::From),
    ("function", Token::Function),
    ("if", Token::If),
    ("import", Token::Import),
    ("in", Token::In),
    ("instanceof", Token::Instanceof),
    ("let", Token::Let),
    ("new", Token::New),
    ("null", Token::Null),
    ("return", Token::Return),
    ("true", Token::True),
    ("typeof", Token::Typeof),
    ("undefined", Token::Undefined),
    ("var", Token::Var),
    ("while", Token::While),
];

fn comment<I>(fst: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = u8>,
{
    if let Some(&ch) = fst.peek() {
        match ch as char {
            '/' => {
                for ch2 in fst.by_ref() {
                    if ch2 as char == '\n' {
                        return Token::Comment;
                    }
                }
            }
            '*' => {
                let mut prev = b'-';
                for ch2 in fst.by_ref() {
                    match ch2 as char {
                        '/' if prev == b'*' => return Token::Comment,
                        _ => prev = ch2,
                    }
                }
            }
            _ => {}
        }
    }
    Token::Operator
}

// todo: not fully covered operator topic
fn operator<I>(fst: &mut Peekable<I>, x: char) -> Token
where
    I: Iterator<Item = u8>,
{
    if let Some(&ch) = fst.peek() {
        match ch as char {
            '*' | '&' | '|' => {
                fst.next();
            }
            _ if x == '*' => {
                return Token::Star;
            }
            _ => {}
        }
    }
    Token::Operator
}

// todo fix plz real numbers
fn number<I>(fst: &mut Peekable<I>)
where
    I: Iterator<Item = u8>,
{
    while let Some(&ch) = fst.peek() {
        match ch as char {
            '0'..='9' => {}
            _ => break,
        }
        fst.next();
    }
}

fn dots<I>(fst: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = u8>,
{
    let mut k = 0;
    for _ in 0..2 {
        if let Some(&ch) = fst.peek() {
            match ch as char {
                '.' => {
                    k += 1;
                }
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
    I: Iterator<Item = u8>,
{
    for ch in fst.by_ref() {
        match ch as char {
            q if q == x => break,
            _ => {}
        }
    }
}

// todo add plz match by underscore
fn variable<I>(fst: &mut Peekable<I>, x: char) -> Token
where
    I: Iterator<Item = u8>,
{
    let mut r = vec![x];
    while let Some(&ch) = fst.peek() {
        match ch as char {
            'A'..='Z' | 'a'..='z' | '0'..='9' => r.push(ch as char),
            _ => break,
        }
        fst.next();
    }
    let s: String = r.iter().collect();
    let i = KEYWORDS.partition_point(|x| x.0 < &s);
    let ok = i != KEYWORDS.len() && KEYWORDS[i].0 == s;
    if ok {
        KEYWORDS[i].1
    } else {
        Token::Variable
    }
}

fn operator_equal<I>(fst: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = u8>,
{
    if let Some(&ch) = fst.peek() {
        match ch as char {
            '=' => {
                fst.next();
                if fst.peek() == Some(&b'=') {
                    fst.next();
                }
                return Token::Operator;
            }
            '>' => {
                fst.next();
                return Token::FatArrow;
            }
            _ => {}
        }
    }
    Token::EqualSign
}

pub fn tokens<I>(mut fst: Peekable<I>) -> Vec<Token>
where
    I: Iterator<Item = u8>,
{
    let mut result = vec![];
    while let Some(ch) = fst.next() {
        match ch as char {
            '0'..='9' => {
                result.push(Token::Number);
                number(&mut fst);
            }
            'A'..='Z' | 'a'..='z' => {
                result.push(variable(&mut fst, ch as char));
            }
            '=' => {
                result.push(operator_equal(&mut fst));
            }
            '-' => result.push(Token::Minus),
            '/' => {
                let t = comment(&mut fst);
                if t != Token::Comment {
                    result.push(t);
                }
            }
            '*' | '+' | '&' | '|' => {
                result.push(operator(&mut fst, ch as char));
            }
            '"' | '\'' => {
                result.push(Token::String);
                string(&mut fst, ch as char);
            }
            '(' => result.push(Token::BracketLeft),
            ')' => result.push(Token::BracketRight),
            '{' => result.push(Token::CurlyBracketLeft),
            '}' => result.push(Token::CurlyBracketRight),
            '[' => result.push(Token::SquareBracketLeft),
            ']' => result.push(Token::SquareBracketRight),
            ';' => result.push(Token::Semicolon),
            ':' => result.push(Token::Colon),
            '?' => result.push(Token::QuestionMark),
            ',' => result.push(Token::Comma),
            '.' => {
                result.push(dots(&mut fst));
            }
            ' ' => {}  // space
            '\n' => {} // space
            _ => {}    // space
        }
    }
    // println!("{:#?}", result);
    result
}
