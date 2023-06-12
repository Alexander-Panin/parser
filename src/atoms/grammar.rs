use std::collections::HashMap;
use std::rc::Rc;

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
    Default,
    Dot,
    Dot2,
    Dot3,
    EqualSign,
    Export,
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
    QuestionMark,
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
    ArrayBuilder,
    Block,
    Call,
    CallBuilder,
    CallTerm,
    ClassBlock,
    ClassBuilder,
    ClosingExpr,
    Condition,
    ExportBuilder,
    ExportExpr,
    ExportTerm,
    Expr,
    ExprMath,
    ExprMathBuilder,
    FunctionBuilder,
    IfBuilder,
    ImportBuilder,
    ImportExpr,
    ImportTerm,
    Lambda,
    Lambda2,
    LambdaBuilder,
    Method,
    MethodBuilder,
    Object,
    ObjectBuilder,
    ObjectValue,
    ReturnBuilder,
    SideEffectBuilder,
    SpreadArray,
    SpreadObject,
    Statement,
    TermArray,
    TermDot,
    TermMath,
    TermObject,
    VariableBuilder,
    WhileBuilder,

    // Backtracing
    BracketLeftBack,
    VariableBack,
    Always,

    // Auxiliary
    Comment,

    #[default]
    Never,
}

#[rustfmt::skip]
pub fn token_tree() -> HashMap<Token, Rc<Node>> {
    use Token::{Expr, TermMath, Assignment, ExprMath, ExprMathBuilder, Statement};
    use Token::{Call, CallTerm, CallBuilder, TermDot, Block, Condition};
    use Token::{Lambda, Lambda2, LambdaBuilder, BracketLeftBack, VariableBack};
    use Token::{FunctionBuilder, IfBuilder, WhileBuilder, ClosingExpr};
    use Token::{ReturnBuilder, SideEffectBuilder};
    use Token::{ClassBuilder, ClassBlock, VariableBuilder, Method, MethodBuilder};
    use Token::{Object, ObjectBuilder, TermObject, Array, ArrayBuilder, TermArray};
    use Token::{SpreadObject, SpreadArray, ObjectValue};
    use Token::{ImportBuilder, ImportExpr, ImportTerm};
    use Token::{ExportBuilder, ExportExpr, ExportTerm};

    let mut expr = HashMap::from([
        (ExprMath, tree![
            | BracketLeft, ExprMath, BracketRight, TermMath
            | Minus, ExprMath
            | Function, FunctionBuilder
            | Number, TermMath
            | String, TermMath
            | Null, TermMath
            | Undefined, TermMath
            | True, TermMath
            | False, TermMath
            | Await, VariableBuilder
            | New, VariableBuilder
            | Typeof, VariableBuilder
            | VariableBuilder
            | Never
        ]),
        (VariableBuilder, tree![
            | Variable, Call, TermDot, TermMath
            | Never
        ]),
        (TermMath, tree![
            | Minus, ExprMath
            | Star, ExprMath
            | Operator, ExprMath
            | QuestionMark, Expr, Colon, Expr
            | Instanceof, ExprMath
            | In, Expr
        ]),
        (ExprMathBuilder, tree![
            | ExprMath
            | Never
        ]),
        (Expr, tree![
            | BracketLeft, Lambda
            | CurlyBracketLeft, Object
            | SquareBracketLeft, Array
            | ExprMathBuilder
        ]),

        // todo `for loop` statement
        (Statement, tree![
            | Import, ImportBuilder, Statement
            | Export, ExportBuilder, Statement
            | Const, Variable, Assignment, Statement
            | Let, Variable, Assignment, Statement
            | Var, Variable, Assignment, Statement
            | Function, FunctionBuilder, Statement
            | Class, ClassBuilder, Statement
            | If, IfBuilder, Statement
            | While, WhileBuilder, Statement
            | Return, ReturnBuilder, Statement
            | Variable, Call, TermDot, SideEffectBuilder, Statement
        ]),
        (FunctionBuilder, tree![
            | Variable, Call, Block
            | Call, Block
            | Never
        ]),
        (ClassBuilder, tree![
            | Variable, ClassBlock
            | CurlyBracketLeft, Method, CurlyBracketRight
            | Never
        ]),
        (ClassBlock, tree![
            | Extends, Variable, CurlyBracketLeft, Method, CurlyBracketRight
            | CurlyBracketLeft, Method, CurlyBracketRight
            | Never
        ]),
        (Method, tree![
            | Variable, MethodBuilder
        ]),
        (MethodBuilder, tree![
            | Call, Block, Method
            | Never
        ]),
        // todo `if` without curly brackets
        (IfBuilder, tree![
            | Condition, Block
            | Never
        ]),
        // todo `while` without curly brackets
        (WhileBuilder, tree![
            | Condition, Block
            | Never
        ]),
        (Condition, tree![
            | BracketLeft, Expr, BracketRight
            | Never
        ]),
        (Block, tree![
            | CurlyBracketLeft, Statement, CurlyBracketRight
            | Statement
            | Never
        ]),
        (ReturnBuilder, tree![
            | Expr, ClosingExpr
            | Never
        ]),
        (SideEffectBuilder, tree![
            | Semicolon
            | EqualSign, Expr, ClosingExpr
        ]),
        (Assignment, tree![
            | EqualSign, Expr, ClosingExpr
            | Never
        ]),
        (TermDot, tree![
            | Dot, Variable, Call, TermDot
        ]),
        (ClosingExpr, tree![
            | Semicolon
        ]),
    ]);

    let call = HashMap::from([
        (Call, tree![
            | BracketLeft, CallBuilder
        ]),
        (CallBuilder, tree![
            | BracketRight
            | Expr, CallTerm, BracketRight
            | Never
        ]),
        (CallTerm, tree![
            | Comma, Expr, CallTerm
        ]),
    ]);

    let literals = HashMap::from([
        (Object, tree![
            | ObjectBuilder, CurlyBracketRight
            | Never
        ]),
        (ObjectBuilder, tree![
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
            | Comma, ObjectBuilder
        ]),
        (SpreadObject, tree![
            | Variable, TermObject
            | CurlyBracketLeft, Object, TermObject
            | Never
        ]),
        (Array, tree![
            | SquareBracketRight
            | ArrayBuilder, SquareBracketRight
            | Never
        ]),
        (ArrayBuilder, tree![
            | Dot3, SpreadArray
            | Expr, TermArray
        ]),
        (TermArray, tree![
            | Comma, ArrayBuilder
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
            | BracketRight, FatArrow, LambdaBuilder
            | Variable, Lambda2
            | BracketLeftBack, ExprMathBuilder
        ]),
        (Lambda2, tree![
            | BracketRight, FatArrow, LambdaBuilder
            | Comma, Variable, CallTerm, BracketRight, FatArrow, LambdaBuilder
            | VariableBack, BracketLeftBack, ExprMathBuilder
        ]),
        (LambdaBuilder, tree![
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

    let import = HashMap::from([
        (ImportBuilder, tree![
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

    // todo maybe later: cover other cases 
    let export = HashMap::from([
        (ExportBuilder, tree![
            | Star, From, String, ClosingExpr
            | CurlyBracketLeft, Variable, ExportTerm, CurlyBracketRight, ClosingExpr
            | Default, ExportExpr
            | ExportExpr
        ]),
        (ExportTerm, tree![
            | Comma, Variable, ExportTerm
        ]),
        (ExportExpr, tree![
            | Class, ClassBuilder
            | Const, Variable, Assignment
            | Function, FunctionBuilder
            | Variable, ClosingExpr
            | Never
        ]),
    ]);

    expr.extend(lambda);
    expr.extend(call);
    expr.extend(literals);
    expr.extend(import);
    expr.extend(export);
    expr
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Node {
    pub val: Token,
    pub ok: Option<Rc<Node>>,
    pub err: Option<Rc<Node>>,
}
