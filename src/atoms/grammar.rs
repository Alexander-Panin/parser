use std::rc::Rc;
use std::collections::HashMap;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug, Eq, Hash)]
pub enum Token {
    Assignment,
    AssignmentOrCall,
    Await,
    BracketLeft,
    BracketRight,
    Call,
    CallBody,
    CallTerm,
    Class,
    ClassBody,
    ClosingExpr,
    Const,
    Comma,
    CurlyBracketLeft,
    CurlyBracketRight,
    Dot,
    DotTerm,
    EqualSign,
    Expr,
    ExprMath,
    Extends,
    FatArrow,
    Function,
    FunctionBody,
    If,
    IfBody,
    Lambda2,
    Lambda3,
    LambdaBody,
    Let,
    Method,
    Minus,
    New,
    Number,
    Operator,
    Return,
    Statement,
    Semicolon,
    TermMath,
    Var,
    Variable,
    VariableBody,
    While,
    WhileBody,

    // Backtracing 
    BracketLeftBack,
    VariableBack,
    Always,

    #[default]
    Never,
}

pub fn token_tree() -> HashMap<Token, Rc<Node>> {
    use Token::{Expr, TermMath, Assignment, ExprMath, Statement };
    use Token::{Call, CallTerm, CallBody, AssignmentOrCall, DotTerm};
    use Token::{Lambda2, Lambda3, LambdaBody, BracketLeftBack, VariableBack};
    use Token::{FunctionBody, IfBody, WhileBody, ClosingExpr};
    use Token::{ClassBody, VariableBody, Method};

    let mut expr = HashMap::from([
        (ExprMath, tree![
            | BracketLeft, ExprMath, BracketRight, TermMath
            | Minus, ExprMath
            | Function, FunctionBody
            | Number, TermMath
            | Await, VariableBody
            | New, VariableBody
            | VariableBody
            | Never
        ]),
        (VariableBody, tree![
            | Variable, Call, TermMath
            | Never
        ]),
        (TermMath, tree![
            | Minus, ExprMath
            | Operator, ExprMath
            | Dot, ExprMath
        ]),
        
        (Expr, tree![ // <- Lambda
            | BracketLeft, Lambda2
            | ExprMath
        ]),
        (Assignment, tree![
            | Variable, EqualSign, Expr, ClosingExpr 
            | Never
        ]),
        (AssignmentOrCall, tree![
            | EqualSign, Expr, ClosingExpr
            | Call, ClosingExpr
        ]),
        (DotTerm, tree![
            | Dot, Variable, Call, DotTerm
        ]),
        (Statement, tree![
            | Function, Variable, FunctionBody, Statement
            | Class, Variable, ClassBody, Statement
            | If, IfBody, Statement
            | While, WhileBody, Statement
            | Return, Expr, ClosingExpr, Statement
            | Const, Assignment, Statement 
            | Let, Assignment, Statement
            | Var, Assignment, Statement
            | Variable, DotTerm, AssignmentOrCall, Statement 
        ]),
        (ClassBody, tree![
            | Extends, Variable, CurlyBracketLeft, Method, CurlyBracketRight 
            | CurlyBracketLeft, Method, CurlyBracketRight 
            | Never
        ]),
        (Method, tree![
            | Variable, FunctionBody, Method
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
    ]);
    
    let lambda = HashMap::from([
        (Lambda2, tree![
            | BracketRight, FatArrow, LambdaBody
            | Variable, Lambda3
            | BracketLeftBack, ExprMath
        ]),
        (Lambda3, tree![
            | BracketRight, FatArrow, LambdaBody
            | Comma, Variable, CallTerm, BracketRight, FatArrow, LambdaBody
            | VariableBack, BracketLeftBack, ExprMath
        ]),
        (LambdaBody, tree![
            | CurlyBracketLeft, Statement, CurlyBracketRight
            | Expr
            | Never
        ]),
        (BracketLeftBack, tree![
            | Always 
        ]),
        (VariableBack, tree![
            | Always
        ]),
    ]);
    let call = HashMap::from([
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
    ]);
    expr.extend(lambda);
    expr.extend(call);
    expr
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}


