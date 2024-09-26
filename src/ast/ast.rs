use std::borrow::Cow;
use crate::lexer::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}

impl Ast {
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Box<Expr>),
    Import(Import),
    Block(Block),
    If(If),
    Return(Return),
    Fn(Fn),
    // TODO: loop, continue, break
}

impl Stmt {
    pub fn new_fn(fn_token: Token, name: String, params: Vec<FnParam>, body: Block, exported: bool, return_type: Option<FunctionType>) -> Self {
        Stmt::Fn(Fn {
            fn_token,
            name,
            params,
            body,
            exported,
            return_type,
        })
    }
}

impl Stmt {
    pub fn as_function(&self) -> &Fn {
        match self {
            Stmt::Fn(f) => f,
            _ => panic!("Expected function"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnParam {
    pub ident: Token,
    pub type_annotation: TypeAnnotation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub colon: Token,
    pub type_name: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub arrow: Token,
    pub type_name: Token,
}

impl FunctionType {
    pub fn new(arrow: Token, type_name: Token) -> Self {
        Self { arrow, type_name }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fn {
    pub fn_token: Token,
    pub name: String,
    pub params: Vec<FnParam>,
    pub body: Block,
    pub exported: bool,
    pub return_type: Option<FunctionType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    pub if_token: Token,
    pub condition: Box<Expr>,
    pub then_block: Block,
    pub else_ifs: Vec<ElseIf>,
    pub else_block: Option<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElseIf {
    pub else_if_token: Token,
    pub condition: Box<Expr>,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub import_token: Token,
    pub from: Token,
    pub items: Vec<Token>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Return {
    pub return_token: Token,
    pub expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralType {
    Int(i64),
    Rational(f64),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    pub token: Token,
    pub value: LiteralType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BinOp {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    Ampersand,
    Pipe,
    Caret,
    DoubleAsterisk,
    Percent,
    Tilde,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    EqualsEquals,
    BangEquals,
    Increment,
    Decrement,
    MinusEquals,
    PlusEquals,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: BinOp,
    pub right: Box<Expr>,
    pub token: Token,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnaryOp {
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub operator: UnaryOp,
    pub expr: Box<Expr>,
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub ident: String,
    pub token: Token,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: LogicalOp,
    pub right: Box<Expr>,
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parenthesized {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: String,
    pub args: Vec<Expr>,
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub ident: Variable,
    pub value: Box<Expr>,
    pub token: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Binary(Binary),
    Unary(Unary),
    Variable(Variable),
    Logical(Logical),
    Parenthesized(Parenthesized),
    Call(CallExpr),
    Assign(Assign),
}
