use std::iter::Peekable;
use super::super::Token;
use super::operator;
use super::variable;

fn comment<I>(fst: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = u8>,
{
    let Some(&ch) = fst.peek() else {
        return Token::Operator;
    };
    match ch as char {
        '/' => {
            for ch2 in fst.by_ref() {
                if ch2 as char == '\n' {
                    break;
                }
            }
            return Token::Comment;
        }
        '*' => {
            let mut prev = b'-';
            for ch2 in fst.by_ref() {
                match ch2 as char {
                    '/' if prev == b'*' => return Token::Comment,
                    _ => prev = ch2,
                }
            }
            panic!("waiting for closing token of multiline comment");
        }
        _ => {}
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
                result.push(variable::run(&mut fst, ch as char));
            }
            '/' => {
                let t = comment(&mut fst);
                if t != Token::Comment {
                    result.push(t);
                }
            }
            '=' => result.push(operator::equal_or_fat_arrow(&mut fst)),
            '-' => result.push(Token::Minus),
            // '~' => ...,
            '*' | '+' | '&' | '|' | '%' | '^' => {
                result.push(operator::math(&mut fst, ch as char));
            }
            '<' | '>' | '!' => {
                result.push(operator::logical(&mut fst, ch as char));
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
