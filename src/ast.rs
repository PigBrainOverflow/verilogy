// Simplified AST
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TranslationUnit {
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Module {
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>, // inputs and outputs are included in the body
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Parameter {
    pub name: Identifier,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Statement {
    Wire {
        name: Identifier,
        width: Option<Range>,
        init: Option<Expression>,
        io: Option<Direction>,
    },
    Assign(AssignDecl),
    Instance {
        name: Identifier,
        module: Identifier,
        params_set: Vec<Bind>,
        ports_set: Vec<Bind>,
    },
    Genvar(Identifier),
    Generate(Vec<Statement>),
    For {
        name: Option<Identifier>,
        init: AssignDecl,
        cond: Expression,
        step: AssignDecl,
        body: Vec<Statement>,
    }, // TODO: add more statement types
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AssignDecl {
    pub name: Identifier,
    pub width: Option<Range>,
    pub value: Expression,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Input,
    Output,
    InOut,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Range {
    pub start: Box<Expression>,
    pub end: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Bind {
    // binds a parameter to an argument or a port to a wire
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expression {
    Identifier(Identifier),
    ConstantInt(i64),
    BinaryBitOperation(Box<Expression>, BinBitOp, Box<Expression>),
    UnaryBitOperation(UnBitOp, Box<Expression>),
    BinaryArithmeticOperation(Box<Expression>, BinArithOp, Box<Expression>),
    UnaryArithmeticOperation(UnArithOp, Box<Expression>),
    Slice(Box<Expression>, Range), // TODO: add more expression types, e.g. logic operations, concatenation, etc.
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BinBitOp {
    And,
    Or,
    Xor,
    Xnor,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnBitOp {
    Not,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BinArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnArithOp {
    Neg,
}

// Implementations
impl Module {
    pub fn with_def(
        id: Identifier,
        params: Option<ParameterSignature>,
        ports: Vec<Port>,
        mut body: Vec<Statement>,
    ) -> Self {
        let params = params.unwrap_or(ParameterSignature(vec![]));
        let mut new_body = vec![];

        // add ports to the body
        for port in ports {
            new_body.push(Statement::Wire {
                name: port.name,
                width: port.width,
                init: None,
                io: Some(port.io),
            });
        }
        new_body.append(&mut body);

        Self {
            name: id,
            params: params.0,
            body: new_body,
        }
    }
}

impl Expression {
    pub fn with_constant(literal: &str) -> Self {
        Expression::ConstantInt(literal.parse().unwrap())
    }
}

// Original AST
// not occurring in the output AST
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ParameterSignature(pub Vec<Parameter>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Port {
    pub name: Identifier,
    pub width: Option<Range>,
    pub io: Direction,
}
