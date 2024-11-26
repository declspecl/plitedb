#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    PutItem { store_name: String, assignments: Vec<Assignment> },
    GetItem { store_name: String, conditions: Vec<Condition> }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Value(Value),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation)
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub operator: BinaryOperator
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    pub operand: Box<Expression>,
    pub operator: UnaryOperator
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Integer(i64),
    String(String),
    Boolean(bool)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub lhs: Expression,
    pub rhs: Expression,
    pub comparison_type: ComparisonOperator
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual
}
