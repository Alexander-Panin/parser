use std::rc::Rc;
use crate::atoms::{Token, TokenTree, Node, tree_length};
use crate::registry::{Registry, ID};

#[derive(Default, PartialEq, Debug)]
pub struct Audit {
    pub registry: Registry<Rc<Node>>,
    pub queue: Vec<ID>,
    pub matcher: Vec<Token>,
    pub tt: TokenTree,
}

impl Audit {
    pub fn double_entry(&mut self, node: Rc<Node>) {
        let t = self.registry.append(node.clone());
        let n = tree_length(Some(node));
        for _ in 0..n { self.queue.push(t); }
    }

    fn booked(&mut self, t: ID, token: Token) -> bool {
        match token {
            Token::Expr => {
                self.double_entry(self.tt.expr.clone());
            },
            Token::Term => {
                self.double_entry(self.tt.term.clone());
            },
            Token::Assignment => {
                self.double_entry(self.tt.assignment.clone());
            },
            Token::IfBody => {
                self.double_entry(self.tt.if_body.clone());
            },
            Token::WhileBody => {
                self.double_entry(self.tt.while_body.clone());
            },
            Token::Statement => {
                self.double_entry(self.tt.statement.clone());
            },
            Token::ClosingExpr => {
                self.double_entry(self.tt.closing_expr.clone());
            },
            Token::FunctionBody => {
                self.double_entry(self.tt.function_body.clone());
            },
            _ => { return false; }, 
        }
        self.boost_entry(t);
        return true;
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

