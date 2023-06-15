use crate::atoms::{tree_length, Choice, Token};
use crate::registry::{Registry, ID};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Default, PartialEq, Debug)]
pub struct Audit {
    pub registry: Registry<Rc<Choice>>,
    pub queue: Vec<ID>,
    pub matcher: Vec<Token>,
    pub tt: HashMap<Token, Rc<Choice>>,
}

impl Audit {
    pub fn double_entry(&mut self, word: Rc<Choice>) {
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

            let Some(w) = self.registry.get(t) else { continue; };
            let Choice::Word(token, _, _) = w.deref() else { continue; };
            if !self.booked(t, *token) {
                let ok = self.approved(t);
                self.audit_step(t, ok);
            }
        }
    }

    fn booked(&mut self, t: ID, token: Token) -> bool {
        let Some(w) = self.tt.get(&token) else { return false; };
        self.double_entry(w.clone());
        self.boost_entry(t);
        self.backtrace(token);
        true
    }

    fn backtrace(&mut self, token: Token) {
        match token {
            Token::BracketLeftBack => self.matcher.push(Token::BracketLeft),
            Token::VariableBack => self.matcher.push(Token::Variable),
            _ => {}
        }
    }

    fn audit_step(&mut self, t: ID, approved: bool) {
        let e = self.registry.get_mut(t).unwrap();
        let Choice::Word(val, ref ok, ref err) = *e.clone() else { return; };
        if val == Token::Never {
            return;
        }

        let x = if approved { ok.clone() } else { err.clone() };
        if *x == Choice::Nil {
            self.registry.erase(t);
            return;
        }
        *e = x;
    }

    fn boost_entry(&mut self, t: ID) {
        let e = self.registry.get_mut(t).unwrap();
        match *e.clone() {
            Choice::Word(_, ref ok, _) if *ok.clone() != Choice::Nil => {
                *e = ok.clone();
            }
            _ => self.registry.erase(t),
        }
    }

    fn approved(&mut self, t: ID) -> bool {
        let e = self.registry.get(t).unwrap();
        let Choice::Word(val, _, _) = *e.clone() else {
            return false;
        };
        let result = Some(val) == self.matcher.last().cloned();
        if result {
            self.matcher.pop();
        }
        result
    }
}
