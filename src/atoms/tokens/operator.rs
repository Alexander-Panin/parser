use super::super::Token;
use std::iter::Peekable;

// todo: not fully covered operator topic
pub fn math<I>(fst: &mut Peekable<I>, x: char) -> Token
where
    I: Iterator<Item = u8>,
{
    if let Some(&ch) = fst.peek() {
        match ch as char {
            '*' | '&' | '|' => {
                fst.next();
            }
            '=' => {
                fst.next();
                return Token::AssignmentOperator;
            }
            _ if x == '*' => {
                return Token::Star;
            }
            _ => {}
        }
    }
    Token::Operator
}

pub fn logical<I>(fst: &mut Peekable<I>, x: char) -> Token
where
    I: Iterator<Item = u8>,
{
    if let Some(&ch) = fst.peek() {
        match ch as char {
            '=' => {
                fst.next();
                if x == '!' && fst.peek() == Some(&b'=') {
                    fst.next();
                }
                return Token::Operator;
            }
            '<' if x == '<' => { fst.next(); }
            '>' if x == '>' => { fst.next(); }
            _ => {}
        }
    }
    if x == '!' { Token::Bang } else { Token::Operator }
}

pub fn equal_or_fat_arrow<I>(fst: &mut Peekable<I>) -> Token
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