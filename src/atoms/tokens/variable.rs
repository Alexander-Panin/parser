use super::super::Token;
use std::iter::Peekable;

// require to be in order
const KEYWORDS: [(&str, Token); 32] = [
    ("as", Token::As),
    ("await", Token::Await),
    ("catch", Token::Catch),
    ("class", Token::Class),
    ("const", Token::Const),
    ("declare", Token::Declare),
    ("default", Token::Default),
    ("else", Token::Else),
    ("export", Token::Export),
    ("extends", Token::Extends),
    ("false", Token::False),
    ("finally", Token::Finally),
    ("for", Token::For),
    ("from", Token::From),
    ("function", Token::Function),
    ("if", Token::If),
    ("import", Token::Import),
    ("in", Token::In),
    ("instanceof", Token::Instanceof),
    ("let", Token::Let),
    ("new", Token::New),
    ("null", Token::Null),
    ("of", Token::Of),
    ("return", Token::Return),
    ("throw", Token::Throw),
    ("true", Token::True),
    ("try", Token::Try),
    ("type", Token::Type),
    ("typeof", Token::Typeof),
    ("undefined", Token::Undefined),
    ("var", Token::Var),
    ("while", Token::While),
];

// todo add plz match by underscore
pub fn run<I>(fst: &mut Peekable<I>, x: char) -> Token
where
    I: Iterator<Item = u8>,
{
    let mut r = vec![x];
    while let Some(&ch) = fst.peek() {
        match ch as char {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '$' => r.push(ch as char),
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
