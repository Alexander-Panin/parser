use crate::atoms::{tree_length, Choice, Word, Token};
use crate::registry::{Registry, ID};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(PartialEq, Debug)]
pub struct Audit<'a> {
    pub registry: Registry<Word>,
    pub queue: Vec<ID>,
    pub matcher: Vec<Token>,
    pub tt: &'a HashMap<Token, Choice>,
}

impl Audit<'_> {
    pub fn double_entry(&mut self, choice: Choice) {
        let n = tree_length(&choice);
        let t = self.registry.append(choice.unwrap());
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

            let Some(Word(token, _, _)) = self.registry.get(t) else {
                continue;
            };
            if !self.booked(t, *token) {
                let is_ok = self.approved(t);
                self.audit_step(t, is_ok);
            }
        }
    }

    fn booked(&mut self, t: ID, token: Token) -> bool {
        let Some(ref choice) = self.tt.get(&token) else {
            return false;
        };
        self.double_entry(choice.deref().clone());
        self.boost_entry(t);
        self.backtrace(token);
        true
    }

    fn approved(&mut self, t: ID) -> bool {
        let Word(val, _, _) = self.registry.get(t).unwrap();
        let result = Some(*val) == self.matcher.last().cloned();
        if result {
            self.matcher.pop();
        }
        result
    }

    fn audit_step(&mut self, t: ID, approved: bool) {
        let word = self.registry.get_mut(t).unwrap();
        let Word(val, ref ok, ref err) = word;
        if *val == Token::Never {
            return;
        }
        let x = if approved { ok } else { err };
        if x.deref().is_none() {
            self.registry.erase(t);
            return;
        }
        *word = x.deref().clone().unwrap();
    }

    fn boost_entry(&mut self, t: ID) {
        let word = self.registry.get_mut(t).unwrap();
        match word {
            Word(_, ref ok, _) if ok.deref().is_some() => {
                *word = ok.deref().clone().unwrap();
            }
            _ => self.registry.erase(t),
        }
    }

    fn backtrace(&mut self, token: Token) {
        match token {
            Token::BracketLeftBack => self.matcher.push(Token::BracketLeft),
            Token::VariableBack => self.matcher.push(Token::Variable),
            _ => {}
        }
    }
}
