use std::rc::Rc;
use std::collections::HashMap;
use crate::atoms::{Token, Node, tree_length};
use crate::registry::{Registry, ID};

#[derive(Default, PartialEq, Debug)]
pub struct Audit {
    pub registry: Registry<Rc<Node>>,
    pub queue: Vec<ID>,
    pub matcher: Vec<Token>,
    pub tt: HashMap<Token, Rc<Node>>,
}

impl Audit {
    pub fn double_entry(&mut self, node: Rc<Node>) {
        let t = self.registry.append(node.clone());
        let n = tree_length(Some(node));
        for _ in 0..n { self.queue.push(t); }
    }

    pub fn audit(&mut self) {
        while let Some(t) = self.queue.pop() {
            // println!("{:#?}", self.registry);
            // println!("{:#?}", self.matcher);
            // println!("{:#?}", self.queue);
            // println!("----------------------");
            if let Some(token) = self.registry.get(t).as_ref().map(|n| n.val) {
                if !self.booked(t, token) {
                    let ok = self.approved(t);
                    self.audit_step(t, ok);
                }
            }
        }
    }

    fn booked(&mut self, t: ID, token: Token) -> bool {
        let r = self.tt.get(&token);
        if r.is_none() { return false; }
        self.double_entry(r.cloned().unwrap());
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

    fn audit_step(&mut self, t: ID, ok: bool) {
        let e = self.registry.get_mut(t).unwrap();
        if e.val == Token::Never { return; }
        let x = if ok { 
            e.ok.as_ref().cloned() 
        } else { 
            e.err.as_ref().cloned() 
        };
        if x.is_none() { 
            self.registry.erase(t); 
            return;
        }
        *e = x.unwrap();
    }

    fn boost_entry(&mut self, t: ID) {
        let e = self.registry.get_mut(t).unwrap();
        if e.ok.is_none() {
            self.registry.erase(t);
            return;
        }
        *e = e.ok.as_ref().unwrap().clone();
    }

    fn approved(&mut self, t: ID) -> bool {
        let x = self.registry.get(t).as_ref().map(|n| n.val);
        let y = self.matcher.last().cloned(); 
        let result = x == y; 
        if result { self.matcher.pop(); }
        result
    }
}

