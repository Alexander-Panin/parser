use std::rc::Rc;
use std::collections::HashMap;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug, Eq, Hash)]
pub enum Token {
    Assignment,
    As,
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
    From,
    Function,
    If,
    Import,
    Instanceof,
    In,
    Let,
    Minus,
    New,
    Null,
    Number,
    Operator,
    Return,
    Star,
    Semicolon,
    SquareBracketLeft,
    SquareBracketRight,
    String,
    True,
    Typeof,
    Undefined,
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
    IfHeader,
    ImportHeader,
    ImportExpr,
    ImportTerm,
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

    // Auxiliary
    Comment,

    #[default]
    Never,
}

pub fn token_tree() -> HashMap<Token, Rc<Node>> {
    use Token::{Expr, TermMath, Assignment, ExprMath, Statement};
    use Token::{Call, CallTerm, CallBody, AssignmentOrCall, TermDot};
    use Token::{Lambda, Lambda2, LambdaBody, BracketLeftBack, VariableBack};
    use Token::{FunctionBody, IfBody, IfHeader, WhileBody, ClosingExpr};
    use Token::{ClassBody, VariableBody, Method};
    use Token::{Object, ObjectBody, TermObject, Array, ArrayBody, TermArray};
    use Token::{SpreadObject, SpreadArray, ObjectValue};
    use Token::{ImportHeader, ImportExpr, ImportTerm};

    let import = HashMap::from([
        (ImportHeader, tree![
            | String, ClosingExpr
            | ImportExpr, From, String, ClosingExpr 
            | Never
        ]),
        (ImportExpr, tree![
            | CurlyBracketLeft, Variable, ImportTerm, CurlyBracketRight
            | Variable, ImportTerm
            | Star, ImportTerm
        ]), 
        (ImportTerm, tree![
            | Comma, ImportExpr
            | As, Variable, ImportTerm
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
            | Undefined, TermMath
            | True, TermMath
            | False, TermMath
            | Await, VariableBody
            | New, VariableBody
            | Typeof, VariableBody
            | VariableBody
            | Never
        ]),
        (VariableBody, tree![
            | Variable, Call, TermMath
            | Never
        ]),
        (TermMath, tree![
            | Minus, ExprMath
            | Star, ExprMath
            | Operator, ExprMath
            | Instanceof, ExprMath
            | In, Expr
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
            | Import, ImportHeader, Statement
            | Function, Variable, FunctionBody, Statement
            | Class, Variable, ClassBody, Statement
            | If, IfHeader, Statement
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
        (IfHeader, tree![
            | BracketLeft, Expr, BracketRight, IfBody
            | Never
        ]),
        (IfBody, tree![
            | CurlyBracketLeft, Statement, CurlyBracketRight
            | Statement
        ]),
        // todo `while` without curly brackets
        (WhileBody, tree![
            | BracketLeft, Expr, BracketRight,
                CurlyBracketLeft, Statement, CurlyBracketRight
            | Never
        ]),
        (ClosingExpr, tree![
            | Semicolon
        ]),
    ]);

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
    expr.extend(import);
    expr
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}


