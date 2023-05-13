use std::rc::Rc;
use crate::atoms::{Token, TokenTree, Node};
use crate::registry::{Registry, ID};

#[derive(Default, PartialEq, Debug)]
pub struct Audit {
    pub registry: Registry<Rc<Node>>,
    pub queue: Vec<ID>,
    pub matcher: Vec<Token>,
    pub tt: TokenTree,
}

impl Audit {
    pub fn double_entry_statement(&mut self) {
        let t = self.registry.append(self.tt.statement.clone());
        for _ in 0..7 { // longest path
            self.queue.push(t); 
        }
    }

    pub fn double_entry_if_body(&mut self) {
        let t = self.registry.append(self.tt.if_body.clone());
        for _ in 0..6 { // longest path
            self.queue.push(t); 
        }
    }

    pub fn double_entry_while_body(&mut self) {
        let t = self.registry.append(self.tt.while_body.clone());
        for _ in 0..6 { // longest path
            self.queue.push(t); 
        }
    }

    pub fn double_entry_assignment(&mut self) {
        let t = self.registry.append(self.tt.assignment.clone());
        for _ in 0..4 { // longest path
            self.queue.push(t); 
        }
    }

    pub fn double_entry_expr(&mut self) {
        let t = self.registry.append(self.tt.expr.clone());
        for _ in 0..6 { // longest path
            self.queue.push(t); 
        }
    }

    pub fn double_entry_term(&mut self) {
        let t = self.registry.append(self.tt.term.clone());
        self.queue.push(t); 
        self.queue.push(t); // longest path of TERM_TREE;
    }

    fn booking(&mut self, t: ID, token: Token) {
        match token {
            Token::Expr => {
                self.double_entry_expr();
            },
            Token::Term => {
                self.double_entry_term();
            },
            Token::Assignment => {
                self.double_entry_assignment();
            },
            Token::IfBody => {
                self.double_entry_if_body();
            },
            Token::WhileBody => {
                self.double_entry_while_body();
            },
            Token::Statement => {
                self.double_entry_statement();
            },
            _ => { return; }, 
        }
        self.boost_entry(t);
    }
    
    pub fn audit(&mut self) {
        while let Some(t) = self.queue.pop() {
            // println!("{:#?}", self.registry);
            // println!("{:#?}", self.matcher);
            // println!("{:#?}", self.queue);
            // println!("----------------------");
            let ok = self.approved(t);
            if let Some(token) = self.audit_step(t, ok) {
                self.booking(t, token);
            }
        }
    }

    fn audit_step(&mut self, t: ID, ok: bool) -> Option<Token> {
        let x = self.registry.get_mut(t);
        if x.is_none() { return None; }
        let e = x.unwrap();
        if e.val == Token::Never {
            return None;
        }
        let y = if ok { 
            e.ok.as_ref().map(|n| n.clone()) 
        } else { e.err.as_ref().map(|n| n.clone()) };
        if y.is_none() {
            self.registry.erase(t);
            return None;
        }
        *e = y.unwrap();
        return Some(e.val);
    }

    fn boost_entry(&mut self, t: ID) {
        let x = self.registry.get_mut(t);
        if x.is_none() { return; }
        let e = x.unwrap();
        if e.ok.is_none() {
            self.registry.erase(t);
            return;
        }
        *e = e.ok.as_ref().unwrap().clone();
    }

    fn approved(&mut self, t: ID) -> bool {
        let x = self.registry.get(t).as_ref().map(|n| n.val);
        let y = self.matcher.last().cloned(); 
        let r = x == y; 
        if r { self.matcher.pop(); }
        return r;    
    }
}

