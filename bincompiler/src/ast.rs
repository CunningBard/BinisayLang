use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Identifier {
    Single(String),
    DotIdentifier(Vec<String>), // variables with dots in them
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Identifier::Single(name) => name.to_string(),
            Identifier::DotIdentifier(names) => names.join("."),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Variable(Identifier),
    FunctionCall {
        func_name: Identifier,
        args: Vec<Expression>,
    },
    Addition {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Subtraction {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiplication {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Division {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Equal {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    NotEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}


#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        identifier: Identifier,
        expression: Expression,
    },
    Reassignment {
        identifier: Identifier,
        expression: Expression,
    },
    FunctionCall {
        func_name: Identifier,
        args: Vec<Expression>,
    },
    FunctionDeclaration {
        func_name: Identifier,
        args: Vec<Identifier>,
        body: Vec<Statement>,
    },
    Conditional {
        body: Vec<(Expression, Vec<Statement>)>,
        else_body: Option<Vec<Statement>>,
    },
    WhileLoop {
        condition: Expression,
        body: Vec<Statement>,
    },
    Break,
    Continue,
    EOI,
    Return(Expression),
    Comment(String),
}