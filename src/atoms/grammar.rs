use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug, Eq, Hash)]
pub enum Token {
    AngleBracketLeft,
    AngleBracketRight,
    Assignment,
    AssignmentOperator,
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
    Declare,
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
    QuestionDotMark,
    Return,
    Star,
    Semicolon,
    SquareBracketLeft,
    SquareBracketRight,
    String,
    Throw,
    True,
    Try,
    Typeof,
    Type,
    Undefined,
    Var,
    Variable,
    While,

    // compound
    ArrayBuilder,
    Block,
    Call,
    CallBuilder,
    CallTerm,
    CallExpr,
    ClassBlock,
    ClassBuilder,
    ClosingExpr,
    Condition,
    ExportBuilder,
    ExportExpr,
    ExportTerm,
    ExportFrom,
    ExportVariable,
    Expr,
    ExprMath,
    ExprMathBuilder,
    FunctionBuilder,
    FnInit,
    FnInitBuilder,
    FnInitTerm,
    FnInitVariable,
    FnInitVariableType,
    FnInitType,
    FnInitTypeTerm,
    FnInitTypeLambda,
    FnInitTypeTemplate,
    FnInitTypeTemplateTerm,
    FnInitVariableDefault,
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
    TermMath,
    TermObject,
    VariableAccess,
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
    VariableDestructuringTerm,
    VariableDestructuring,
    VariableDestructuringInside,
    VariableDestructuringNamed,
    ThrowBuilder,

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
        Call, CallTerm, CallExpr, CallBuilder, Block, Condition,
        Lambda, Lambda2, LambdaBuilder, BracketLeftBack, VariableBack,
        FunctionBuilder, FnInit, FnInitBuilder, FnInitTerm, FnInitVariable,
        FnInitVariableType, FnInitType, FnInitTypeTerm, FnInitVariableDefault, 
        FnInitTypeTemplateTerm, FnInitTypeTemplate, FnInitTypeLambda,
        IfBuilder, WhileBuilder, ClosingExpr,
        ReturnBuilder, SideEffectBuilder, VariableAccess,
        ClassBuilder, ClassBlock, VariableBuilder, Method, MethodBuilder,
        Object, ObjectBuilder, TermObject, ArrayBuilder, TermArray,
        SpreadObject, SpreadArray, ObjectValue, ThrowBuilder,
        ImportBuilder, ImportExpr, ImportTerm,
        ExportBuilder, ExportExpr, ExportTerm, ExportFrom, ExportVariable,
        CatchBuilder, FinallyBuilder, TryBuilder, 
        ForBuilder, ForCondition, ForConditionAssignment,
        ForConditionInside, ForConditionNext, ForConditionNext2,
        VariableDestructuringTerm, VariableDestructuringInside,
        VariableDestructuring, VariableDestructuringNamed
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
            | AngleBracketLeft, ExprMath
            | AngleBracketRight, ExprMath
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
            | SquareBracketLeft, ArrayBuilder
            | ExprMathBuilder
        ]),
        (VariableBuilder, tree![
            | Variable, Call, VariableAccess, TermMath
            | Never
        ]),
        (VariableAccess, tree![
            | QuestionDotMark, Variable, Call, VariableAccess
            | Dot, Variable, Call, VariableAccess
            | SquareBracketLeft, Expr, SquareBracketRight, Call, VariableAccess
        ]),

        (VariableDestructuringNamed, tree![
            | Colon, Variable, VariableDestructuringTerm
        ]),
        (VariableDestructuringTerm, tree![
            | Comma, VariableDestructuringInside
        ]),
        (VariableDestructuringInside, tree![
            | Dot3, Variable, VariableDestructuringTerm
            | Variable, VariableDestructuringNamed, VariableDestructuringTerm
        ]),
        (VariableDestructuring, tree![
            | CurlyBracketLeft, VariableDestructuringInside, CurlyBracketRight
            | SquareBracketLeft, VariableDestructuringInside, SquareBracketRight 
            | Variable
        ]),
        (Statement, tree![
            | Import, ImportBuilder, Statement
            | Export, ExportBuilder, Statement
            | Const, VariableDestructuring, Assignment, Statement
            | Let, VariableDestructuring, Assignment, Statement
            | Var, VariableDestructuring, Assignment, Statement
            | Function, FunctionBuilder, Statement
            | Declare, Function, Variable, FnInit, ClosingExpr, Statement
            | Class, ClassBuilder, Statement
            | If, IfBuilder, Statement
            | While, WhileBuilder, Statement
            | For, ForBuilder, Statement
            | Return, ReturnBuilder, Statement
            | Try, TryBuilder, Statement
            | Throw, ThrowBuilder, Statement
            | Variable, Call, VariableAccess, SideEffectBuilder, Statement
        ]),
        (Assignment, tree![
            | EqualSign, Expr, ClosingExpr
            | Never
        ]),
        (ClosingExpr, tree![
            | Semicolon
        ]),
    ]);

    let builders = HashMap::from([
        (FunctionBuilder, tree![
            | Variable, FnInit, Block
            | FnInit, Block
            | Never
        ]),
        (TryBuilder, tree![
            | CurlyBracketLeft, Statement, CurlyBracketRight, CatchBuilder 
            | Never
        ]),
        (ThrowBuilder, tree![
            | New, VariableBuilder, ClosingExpr
            | VariableBuilder, ClosingExpr
            | Never
        ]),
        (CatchBuilder, tree![
            | Catch, FnInit, CurlyBracketLeft, Statement, CurlyBracketRight, FinallyBuilder
            | Never
        ]),
        (FinallyBuilder, tree![
            | Finally, CurlyBracketLeft, Statement, CurlyBracketRight
        ]),
        (IfBuilder, tree![
            | Condition, Block
            | Never
        ]),
        (ForBuilder, tree![
            | ForCondition, Block
            | Never
        ]),
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
            | AssignmentOperator, Expr, ClosingExpr
        ]),
    ]);

    let fn_init = HashMap::from([
        (FnInit, tree![
            | FnInitTypeTemplate, BracketLeft, FnInitBuilder
        ]),
        (FnInitBuilder, tree![
            | BracketRight, FnInitVariableType
            | FnInitVariable, BracketRight, FnInitVariableType
            | Never
        ]),
        (FnInitVariable, tree![
            | Variable, FnInitVariableType, FnInitVariableDefault, FnInitTerm
        ]),
        (FnInitVariableDefault, tree![
            | EqualSign, Expr
        ]),
        (FnInitTerm, tree![
            | Comma, FnInitVariable
        ]),
        (FnInitVariableType, tree![
            | Colon, FnInitType
            | QuestionMark, Colon, FnInitType 
        ]),
        (FnInitType, tree![
            | BracketLeft, FnInitTypeLambda
            | Null, FnInitTypeTerm
            | Variable, FnInitTypeTemplate, FnInitTypeTerm 
            | QuestionMark, Variable, FnInitTypeTerm 
            | Never 
        ]),
        (FnInitTypeLambda, tree![
            | BracketLeft, Lambda, BracketRight, FnInitTypeTerm
            | Lambda, FnInitTypeTerm 
            | Never
        ]),
        (FnInitTypeTerm, tree![
            | Operator, FnInitType 
        ]),
        (FnInitTypeTemplate, tree![
            | AngleBracketLeft, FnInitType, FnInitTypeTemplateTerm, AngleBracketRight  
        ]),
        (FnInitTypeTemplateTerm, tree![
            | Comma, FnInitType
        ]),
    ]);    

    let call = HashMap::from([
        (Call, tree![
            | BracketLeft, CallBuilder
        ]),
        (CallBuilder, tree![
            | BracketRight
            | Expr, CallTerm
            | Never
        ]),
        (CallTerm, tree![
            | BracketRight
            | Comma, CallExpr
            | Never
        ]),
        (CallExpr, tree![
            | BracketRight
            | Expr, CallTerm
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
            | FnInit, Block, Method
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
        (ArrayBuilder, tree![
            | SquareBracketRight
            | Dot3, SpreadArray
            | Expr, TermArray
            | Never
        ]),
        (TermArray, tree![
            | SquareBracketRight
            | Comma, ArrayBuilder
        ]),
        (SpreadArray, tree![
            | SquareBracketLeft, ArrayBuilder, TermArray
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
            | Comma, Variable, FnInitTerm, BracketRight, FatArrow, LambdaBuilder
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
            | Type, ImportExpr, From, String, ClosingExpr
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
            | CurlyBracketLeft, Variable, ExportTerm, CurlyBracketRight, ExportFrom
            | Default, ExportExpr
            | ExportExpr
        ]),
        (ExportFrom, tree![
            | From, String, ClosingExpr
            | ClosingExpr
        ]),
        (ExportTerm, tree![
            | Comma, ExportVariable
            | As, Variable, ExportTerm
        ]),
        (ExportVariable, tree![
            | Variable, ExportTerm
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
    expr.extend(fn_init);
    expr.extend(literals);
    expr.extend(import);
    expr.extend(export);
    expr
}

#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Word(pub Token, pub Arc<Choice>, pub Arc<Choice>);
pub type Choice = Option<Word>;
