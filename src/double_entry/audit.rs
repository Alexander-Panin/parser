use crate::atoms::{tree_length, Choice, Token};
use crate::registry::{Registry, ID};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Default, PartialEq, Debug)]
pub struct Audit {
    pub registry: Registry<Choice>,
    pub queue: Vec<ID>,
    pub matcher: Vec<Token>,
    pub tt: HashMap<Token, Choice>,
}

impl Audit {
    pub fn double_entry(&mut self, word: Choice) {
        let n = tree_length(&word);
        let t = self.registry.append(word);
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

            let Some(Choice::Word(token, _, _)) = self.registry.get(t) else {
                continue;
            };
            if !self.booked(t, *token) {
                let is_ok = self.approved(t);
                self.audit_step(t, is_ok);
            }
        }
    }

    fn booked(&mut self, t: ID, token: Token) -> bool {
        let Some(word) = self.tt.get(&token) else {
            return false;
        };
        self.double_entry(word.clone());
        self.boost_entry(t);
        self.backtrace(token);
        true
    }

    fn approved(&mut self, t: ID) -> bool {
        let Some(Choice::Word(val, _, _)) = self.registry.get(t) else {
            return false;
        };
        let result = Some(*val) == self.matcher.last().cloned();
        if result {
            self.matcher.pop();
        }
        result
    }

    fn audit_step(&mut self, t: ID, approved: bool) {
        let word = self.registry.get_mut(t).unwrap();
        let Choice::Word(val, ref ok, ref err) = word else { return; };
        if *val == Token::Never {
            return;
        }
        let x = if approved { ok } else { err };
        if x.deref() == &Choice::Nil {
            self.registry.erase(t);
            return;
        }
        *word = x.deref().clone();
    }

    fn boost_entry(&mut self, t: ID) {
        let word = self.registry.get_mut(t).unwrap();
        match word {
            Choice::Word(_, ref ok, _) if ok.deref() != &Choice::Nil => {
                *word = *ok.clone();
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
