use std::rc::Rc;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Token {
    Assignment,
    BracketLeft,
    BracketRight,
    Const,
    CurlyBracketLeft,
    CurlyBracketRight,
    EqualSign,
    Expr,
    If,
    IfBody,
    Let,
    Minus,
    Number,
    Operator,
    Statement,
    Term,
    Var,
    Variable,
    While,
    WhileBody,

    #[default]
    Never,
}

pub fn token_tree() -> TokenTree {
    TokenTree {
        expr: tree![
            | BracketLeft, Expr, BracketRight, Term
            | Minus, Expr
            | Number, Term
            | Variable, Term
            | Never
        ],
        term: tree![
            | Minus, Expr
            | Operator, Expr
        ],
        assignment: tree![
            | Variable, EqualSign, Expr 
            | Never
        ],
        statement: tree![
            | If, IfBody
            | While, WhileBody
            | Const, Assignment 
            | Let, Assignment
            | Var, Assignment
            | Variable, EqualSign, Expr 
        ],
        if_body: tree![
            | BracketLeft, Expr, BracketRight,
                CurlyBracketLeft, Statement, CurlyBracketRight
            | Never
        ],
        while_body: tree![
            | BracketLeft, Expr, BracketRight,
                CurlyBracketLeft, Statement, CurlyBracketRight
            | Never
        ],
    }
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct TokenTree {
    pub expr: Rc<Node>,
    pub term: Rc<Node>,        
    pub assignment: Rc<Node>,        
    pub statement: Rc<Node>,        
    pub if_body: Rc<Node>,        
    pub while_body: Rc<Node>,
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}
