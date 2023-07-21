use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug, Eq, Hash)]
pub enum Token {
    Assignment,
    As,
    Await,
    Bang,
    BracketLeft,
    BracketRight,
    Catch,
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
    Finally,
    For,
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
    Of,
    Operator,
    QuestionMark,
    Return,
    Star,
    Semicolon,
    SquareBracketLeft,
    SquareBracketRight,
    String,
    True,
    Try,
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
    FinallyBuilder,
    CatchBuilder,
    TryBuilder,
    ForBuilder,
    ForCondition,
    ForConditionInside,
    ForConditionNext,
    ForConditionNext2,
    ForConditionAssignment,

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
pub fn token_tree() -> HashMap<Token, Choice> {
    use Token::{
        Expr, TermMath, Assignment, ExprMath, ExprMathBuilder, Statement,
        Call, CallTerm, CallBuilder, TermDot, Block, Condition,
        Lambda, Lambda2, LambdaBuilder, BracketLeftBack, VariableBack,
        FunctionBuilder, IfBuilder, WhileBuilder, ClosingExpr,
        ReturnBuilder, SideEffectBuilder,
        ClassBuilder, ClassBlock, VariableBuilder, Method, MethodBuilder,
        Object, ObjectBuilder, TermObject, Array, ArrayBuilder, TermArray,
        SpreadObject, SpreadArray, ObjectValue,
        ImportBuilder, ImportExpr, ImportTerm,
        ExportBuilder, ExportExpr, ExportTerm,
        CatchBuilder, FinallyBuilder, TryBuilder, 
        ForBuilder, ForCondition, ForConditionAssignment,
        ForConditionInside, ForConditionNext, ForConditionNext2
    };

    let mut expr = HashMap::from([
        (ExprMath, tree![
            | BracketLeft, ExprMath, BracketRight, TermMath
            | Minus, ExprMath
            | Bang, ExprMath
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
        (VariableBuilder, tree![
            | Variable, Call, TermDot, TermMath
            | Never
        ]),

        // todo `for of loop` 
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
            | For, ForBuilder, Statement
            | Return, ReturnBuilder, Statement
            | Try, TryBuilder, Statement
            | Variable, Call, TermDot, SideEffectBuilder, Statement
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

    let builders = HashMap::from([
        (FunctionBuilder, tree![
            | Variable, Call, Block
            | Call, Block
            | Never
        ]),
        (TryBuilder, tree![
            | CurlyBracketLeft, Statement, CurlyBracketRight, CatchBuilder 
            | Never
        ]),
        (CatchBuilder, tree![
            | Catch, Call, CurlyBracketLeft, Statement, CurlyBracketRight, FinallyBuilder
            | Never
        ]),
        (FinallyBuilder, tree![
            | Finally, CurlyBracketLeft, Statement, CurlyBracketRight
        ]),
        // todo `if` without curly brackets
        (IfBuilder, tree![
            | Condition, Block
            | Never
        ]),
        // todo `for` without curly brackets
        (ForBuilder, tree![
            | ForCondition, Block
            | Never
        ]),
        // todo `while` without curly brackets
        (WhileBuilder, tree![
            | Condition, Block
            | Never
        ]),
        (ForCondition, tree![
            | BracketLeft, ForConditionInside, BracketRight, Block
            | Never
        ]),
        (ForConditionInside, tree![
            | Semicolon, ForConditionNext
            | Const, Variable, ForConditionAssignment
            | Let, Variable, ForConditionAssignment
            | Var, Variable, ForConditionAssignment
            | Never
        ]),
        (ForConditionAssignment, tree![
            | Of, Variable
            | Assignment, ForConditionNext
            | Never
        ]),
        (ForConditionNext, tree![
            | Semicolon, ForConditionNext2
            | Expr, Semicolon, ForConditionNext2
        ]),
        (ForConditionNext2, tree![
            | Statement
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

    let class = HashMap::from([
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

    expr.extend(builders);
    expr.extend(class);
    expr.extend(lambda);
    expr.extend(call);
    expr.extend(literals);
    expr.extend(import);
    expr.extend(export);
    expr
}

#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Word(pub Token, pub Arc<Choice>, pub Arc<Choice>);
pub type Choice = Option<Word>;
