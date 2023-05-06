use std::rc::Rc;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Token {
    Assignment,
    BracketLeft,
    BracketRight,
    CurlyBracketLeft,
    CurlyBracketRight,
    EqualSign,
    Expr,
    ExprBody,
    Number,
    MinusOperator,
    Operator,
    Const,
    Let,
    Statement,
    If,
    IfBody,
    While,
    WhileBody,
    Term,
    Variable,

    #[default]
    Never,
}

pub fn token_tree() -> TokenTree {
    TokenTree {
        expr: tree![
            | BracketLeft, Expr, BracketRight, Term
            | MinusOperator, ExprBody
            | Number, Term
            | Variable, Term
            | Never
        ],
        term: tree![
            | MinusOperator, Expr
            | Operator, Expr
        ],
        expr_body: tree![
            | Number, Term
            | Variable, Term
            | Never
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
            | Variable, EqualSign, Expr 
            | Never
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
    pub expr_body: Rc<Node>,
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}
