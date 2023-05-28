use std::rc::Rc;
use std::collections::HashMap;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug, Eq, Hash)]
pub enum Token {
    Assignment,
    Await,
    BracketLeft,
    BracketRight,
    Class,
    Colon,
    Comma,
    Const,
    CurlyBracketLeft,
    CurlyBracketRight,
    Dot,
    Dot2,
    Dot3,
    EqualSign,
    Extends,
    False,
    FatArrow,
    Function,
    If,
    Let,
    Minus,
    New,
    Null,
    Number,
    Operator,
    Return,
    Semicolon,
    SquareBracketLeft,
    SquareBracketRight,
    String,
    True,
    Var,
    Variable,
    While,

    // compound
    Array,
    AssignmentOrCall,
    Call,
    CallBody,
    CallTerm,
    ClassBody,
    ClosingExpr,
    Expr,
    ExprMath,
    FunctionBody,
    IfBody,
    Lambda,
    Lambda2,
    LambdaBody,
    Method,
    Object,
    SpreadArray,
    SpreadObject,
    Statement,
    ArrayBody,
    TermArray,
    TermDot,
    TermMath,
    ObjectBody,
    ObjectValue,
    TermObject,
    VariableBody,
    WhileBody,

    // Backtracing 
    BracketLeftBack,
    VariableBack,
    Always,

    #[default]
    Never,
}

pub fn token_tree() -> HashMap<Token, Rc<Node>> {
    use Token::{Expr, TermMath, Assignment, ExprMath, Statement};
    use Token::{Call, CallTerm, CallBody, AssignmentOrCall, TermDot};
    use Token::{Lambda, Lambda2, LambdaBody, BracketLeftBack, VariableBack};
    use Token::{FunctionBody, IfBody, WhileBody, ClosingExpr};
    use Token::{ClassBody, VariableBody, Method};
    use Token::{Object, ObjectBody, TermObject, Array, ArrayBody, TermArray};
    use Token::{SpreadObject, SpreadArray, ObjectValue};

    let literals = HashMap::from([
        (Object, tree![
            | ObjectBody, CurlyBracketRight  
            | Never
        ]),
        (ObjectBody, tree![
            | Variable, ObjectValue
            | String, ObjectValue
            | SquareBracketLeft, Variable, SquareBracketRight, ObjectValue
            | Dot3, SpreadObject
        ]),
        (ObjectValue, tree![
            | Colon, Expr, TermObject
            | Never
        ]),
        (TermObject, tree![
            | Comma, ObjectBody
        ]),
        (SpreadObject, tree![
            | Variable, TermObject
            | CurlyBracketLeft, Object, TermObject
            | Never
        ]),
        (Array, tree![
            | SquareBracketRight
            | ArrayBody, SquareBracketRight
            | Never
        ]),
        (ArrayBody, tree![
            | Dot3, SpreadArray
            | Expr, TermArray
        ]),
        (TermArray, tree![
            | Comma, ArrayBody
        ]),
        (SpreadArray, tree![
            | SquareBracketLeft, Array, TermArray
            | Variable, TermArray
            | Never
        ]),
    ]);

    let mut expr = HashMap::from([
        (ExprMath, tree![
            | BracketLeft, ExprMath, BracketRight, TermMath
            | Minus, ExprMath
            | Function, FunctionBody
            | Number, TermMath
            | String, TermMath
            | Null, TermMath
            | True, TermMath
            | False, TermMath
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
        
        (Expr, tree![ 
            | BracketLeft, Lambda
            | CurlyBracketLeft, Object
            | SquareBracketLeft, Array
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
        (TermDot, tree![
            | Dot, Variable, Call, TermDot
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
            | Variable, TermDot, AssignmentOrCall, Statement 
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
        // todo if without curly brackets
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

    // TODO cover plz lambda with out brackets around single param 
    let lambda = HashMap::from([
        (Lambda, tree![
            | BracketRight, FatArrow, LambdaBody
            | Variable, Lambda2
            | BracketLeftBack, ExprMath
        ]),
        (Lambda2, tree![
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
    expr.extend(literals);
    expr
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}


