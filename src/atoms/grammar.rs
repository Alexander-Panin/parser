use std::rc::Rc;
use std::collections::HashMap;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug, Eq, Hash)]
pub enum Token {
    Assignment,
    BracketLeft,
    BracketRight,
    Call,
    CallBody,
    CallTerm,
    ClosingExpr,
    Const,
    Comma,
    CurlyBracketLeft,
    CurlyBracketRight,
    EqualSign,
    Expr,
    Function,
    FunctionBody,
    If,
    IfBody,
    Let,
    Minus,
    Number,
    Operator,
    Statement,
    Semicolon,
    Term,
    Var,
    Variable,
    While,
    WhileBody,

    #[default]
    Never,
}

pub fn token_tree() -> HashMap<Token, Rc<Node>> {
    use Token::{Expr, Term, Assignment, Statement};
    use Token::{Call, CallTerm, CallBody};
    use Token::{FunctionBody, IfBody, WhileBody, ClosingExpr};
    HashMap::from([
        (Expr, tree![
            | BracketLeft, Expr, BracketRight, Term
            | Minus, Expr
            | Function, FunctionBody
            | Number, Term
            | Variable, Call, Term
            | Never
        ]),
        (Term, tree![
            | Minus, Expr
            | Operator, Expr
        ]),
        (Assignment, tree![
            | Variable, EqualSign, Expr, ClosingExpr 
            | Never
        ]),
        (Statement, tree![
            | If, IfBody, Statement
            | While, WhileBody, Statement
            | Const, Assignment, Statement 
            | Let, Assignment, Statement
            | Var, Assignment, Statement
            | Variable, EqualSign, Expr, ClosingExpr, Statement 
        ]),
        (Call, tree![
            | BracketLeft, CallBody
        ]),
        (CallBody, tree![
            | BracketRight
            | Expr, CallTerm, BracketRight
            | Never
        ]),
        (CallTerm, tree![
            | Comma, Expr, CallTerm
        ]),
        (FunctionBody, tree![
            | Call, CurlyBracketLeft, Statement, CurlyBracketRight
            | Never
        ]),
        (IfBody, tree![
            | BracketLeft, Expr, BracketRight,
                CurlyBracketLeft, Statement, CurlyBracketRight
            | Never
        ]),
        (WhileBody, tree![
            | BracketLeft, Expr, BracketRight,
                CurlyBracketLeft, Statement, CurlyBracketRight
            | Never
        ]),
        (ClosingExpr, tree![
            | Semicolon
        ]),
    ])
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}


