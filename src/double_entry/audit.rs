use crate::atoms::{Token, TokenTree, Cursor};
use crate::registry::{Registry, ID};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Audit<'a> {
    pub registry: Registry<Cursor<'a, Token>>,
    pub queue: Vec<ID>,
    pub matcher: Vec<Token>,
    pub stats: HashMap<Token, usize>,
    pub tt: &'a HashMap<Token, TokenTree>,
}

impl<'a> Audit<'a> {
    pub fn new(
        matcher: Vec<Token>,
        tt: &'a HashMap<Token, TokenTree>
    ) -> Self {
        Self {
            matcher,
            tt,
            registry: Registry::default(),
            queue: vec![],
            stats: HashMap::new(),
        }
    }
}

impl<'a> Audit<'a> {
    pub fn double_entry(&mut self, cursor: Cursor<'a, Token>, n: usize) {
        let t = self.registry.append(cursor);
        for _ in 0..n {
            self.queue.push(t);
        }
    }

    pub fn audit(&mut self) {
        while let Some(t) = self.queue.pop() {
            // println!("{:#?}", self.registry);
            // println!("{:#?}", self.matcher);
            // println!("{:#?}", self.queue);
            // println!("----------------------");

            let Some(cursor) = self.registry.get(t) else {
                continue;
            };
            if !self.booked(t, *cursor.get().unwrap()) {
                let is_ok = self.approved(t);
                self.audit_step(t, is_ok);
            }
        }
    }

    fn booked(&mut self, t: ID, token: Token) -> bool {
        let Some(tree) = self.tt.get(&token) else {
            return false;
        };
        let cursor = tree.cursor();
        self.double_entry(cursor, tree.len);
        self.boost_entry(t);
        self.backtrace(token);
        self.stats(token);
        true
    }

    fn approved(&mut self, t: ID) -> bool {
        let cursor = self.registry.get(t).unwrap();
        let token = *cursor.get().unwrap();
        let is_match = Some(token) == self.matcher.last().cloned(); 
        let is_any = token == Token::AnyToken;
        let is_empty = self.matcher.is_empty();
        let result = !is_empty && (is_match || is_any);
        if result {
            self.matcher.pop();
        }
        result
    }

    fn audit_step(&mut self, t: ID, approved: bool) {
        let cursor = self.registry.get_mut(t).unwrap();
        if *cursor.get().unwrap() == Token::Never {
            return;
        }
        if approved { cursor.left(); } else { cursor.right(); };
        if cursor.get().is_none() { self.registry.erase(t); }
    }

    fn boost_entry(&mut self, t: ID) {
        let cursor = self.registry.get_mut(t).unwrap();
        cursor.left();
        if cursor.get().is_none() { 
            self.registry.erase(t); 
        }
    }

    fn backtrace(&mut self, token: Token) {
        match token {
            Token::BracketLeftBack => self.matcher.push(Token::BracketLeft),
            Token::BracketRightBack => self.matcher.push(Token::BracketRight),
            Token::VariableBack => self.matcher.push(Token::Variable),
            _ => {}
        }
    }    

    fn stats(&mut self, token: Token) {
        self.stats.entry(token)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}
