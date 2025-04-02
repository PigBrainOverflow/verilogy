#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TranslationUnit {
    pub modules: Vec<Module>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Module {
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>    // inputs and outputs are included in the body
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Parameter {
    pub name: Identifier,
    pub value: Option<Expression>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Statement {
    Wire(WireDecl),
    Assign(AssignDecl),
    Instance(InstanceDecl),
    Generate(GenerateBlock),
    For(ForBlock),
    // TODO: add more statement types
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WireDecl {
    pub name: Identifier,
    pub width: Option<Range>,
    pub init: Option<Expression>,
    pub io: Option<Direction>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Input,
    Output,
    Inout
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Range {
    pub start: Expression,
    pub end: Option<Expression>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AssignDecl {
    pub lhs: Expression,
    pub rhs: Expression
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InstanceDecl {
    pub name: Identifier,
    pub module: Identifier,
    pub params_set: Vec<Bind>,
    pub ports_set: Vec<Bind>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Bind {   // binds a parameter to an argument
    pub name: Identifier,
    pub value: Expression
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GenerateBlock {
    pub body: Vec<Statement>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ForBlock {
    pub name: Option<Identifier>,
    pub init: AssignDecl,
    pub cond: Expression,
    pub step: AssignDecl,
    pub body: Vec<Statement>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expression {
    Identifier(Identifier),
    ConstantInt(i64),
    String(String),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
    UnaryOp(Operator, Box<Expression>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr
}