use std::iter::{Peekable};

mod atoms;
mod registry;
mod double_entry;

use atoms::{Token, token_tree};
use double_entry::{Audit};

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

const KEYWORDS: [(&str, Token); 4] = [
    ("const", Token::Const), 
    ("if", Token::If), 
    ("let", Token::Let),
    ("while", Token::While),
];

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
    let ok = i != KEYWORDS.len() && KEYWORDS[i].0 == &s;
    if ok { KEYWORDS[i].1 } else { Token::Variable }
}


fn tokens<I>(mut fst: Peekable<I>) -> Vec<Token> 
    where 
        I: Iterator<Item=u8> {
    let mut r = vec![];
    while let Some(ch) = fst.next() {
        match ch as char {
            '0'..='9' => {
                r.push(Token::Number);
                number(&mut fst);
            },
            'A'..='Z' | 'a'..='z' => {
                r.push(variable(&mut fst, ch as char));
            },
            '=' => r.push(Token::EqualSign),
            '-' => r.push(Token::MinusOperator),
            '*' | '+' | '/' => r.push(Token::Operator),
            '(' => r.push(Token::BracketLeft),
            ')' => r.push(Token::BracketRight),
            '{' => r.push(Token::CurlyBracketLeft),
            '}' => r.push(Token::CurlyBracketRight),
            ' ' => {}, // space 
            _ => {}, // space 
        }
    }
    println!("{:#?}", r);
    return r;
}


fn main() {
    let mut state = Audit::default();
    state.tt = token_tree();
    // let mut ts = tokens("(1+2)+(3+5)*2+5");
    // let mut ts = tokens("x = (((1+2)+(3+5))*2)+5".bytes().peekable());
    // let mut ts = tokens("(((1+2)+(3+5))*2+5"); // invalid
    // let mut ts = tokens("const abZc123 = -11 + 33 * 25 - 5".bytes().peekable());
    let mut ts = tokens("if (1) { while(-a+2) { const a = 5 } }".bytes().peekable());
    ts.reverse();
    state.matcher = ts;
    state.double_entry_statement();
    state.audit();
    println!("done {:?}", state.registry);
}










