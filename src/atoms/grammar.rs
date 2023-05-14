use std::rc::Rc;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug)]
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

pub fn token_tree() -> TokenTree {
    TokenTree {
        expr: tree![
            | BracketLeft, Expr, BracketRight, Term
            | Minus, Expr
            | Function, FunctionBody
            | Number, Term
            | Variable, Call, Term
            | Never
        ],
        term: tree![
            | Minus, Expr
            | Operator, Expr
        ],
        assignment: tree![
            | Variable, EqualSign, Expr, ClosingExpr 
            | Never
        ],
        statement: tree![
            | If, IfBody, Statement
            | While, WhileBody, Statement
            | Const, Assignment, Statement 
            | Let, Assignment, Statement
            | Var, Assignment, Statement
            | Variable, EqualSign, Expr, ClosingExpr, Statement 
        ],
        closing_expr: tree![
            | Semicolon
        ],
        call: tree![
            | BracketLeft, CallBody
        ],
        call_body: tree![
            | BracketRight
            | Expr, CallTerm, BracketRight
            | Never
        ],
        call_term: tree![
            | Comma, Expr, CallTerm
        ],
        function_body: tree![
            | Call, CurlyBracketLeft, Statement, CurlyBracketRight
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
    pub assignment: Rc<Node>,        
    pub closing_expr: Rc<Node>,
    pub expr: Rc<Node>,
    pub function_body: Rc<Node>,
    pub if_body: Rc<Node>,        
    pub statement: Rc<Node>,        
    pub term: Rc<Node>,        
    pub while_body: Rc<Node>,
    pub call: Rc<Node>,
    pub call_body: Rc<Node>,
    pub call_term: Rc<Node>,
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}


