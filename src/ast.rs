use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Invoke {
        fn_name: String,
        args: Vec<Expr>,
    },

    Binary {
        operator: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    Unary {
        operator: Operator,
        operand: Box<Expr>,
    },

    Conditional {
        condition: Box<Expr>,
        consequent: Box<Expr>,
        alternative: Box<Expr>,
    },

    ValDec {
        name: String,
        type_desc: Option<TypeDesc>,
        value: Box<Expr>,
    },

    LetIn {
        val_decls: Vec<Expr>,
        body: Box<Expr>,
    },

    Float(f64),
    StringLiteral(String),
    Int(i32),
    BoolTrue,
    BoolFalse,
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Not,
    And,
    Or,
    Assign,
    Negate,
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Deref,
}

#[derive(Debug, PartialEq)]
pub struct TypedIdentifier {
    pub name: String,
    pub type_desc: TypeDesc,
}

#[derive(Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<TypedIdentifier>,
    pub return_type: TypeDesc,
}

#[derive(Debug, PartialEq)]
pub enum TypeDesc {
    TypeName(String),
    FnSignature {
        args: Vec<TypeDesc>,
        return_type: Box<TypeDesc>,
    },
    IntType,
    FloatType,
    StrType,
    BoolType,
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Box<Expr>,
}

#[derive(Debug)]
pub struct Module {
    pub functions: HashMap<String, Function>,
    pub name: String,
}
