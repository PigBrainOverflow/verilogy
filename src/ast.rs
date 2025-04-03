// Simplified AST
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
    Wire {
        pub name: Identifier,
        pub width: Option<Range>,
        pub init: Option<Expression>,
        pub io: Option<Direction>
    },
    Assign {
        pub lhs: Identifier,
        pub width: Option<Range>,
        pub rhs: Expression
    },
    Instance {
        pub name: Identifier,
        pub module: Identifier,
        pub params_set: Vec<Bind>,
        pub ports_set: Vec<Bind>
    },
    Generate(pub Vec<Statement>),
    For {
        pub name: Option<Identifier>,
        pub init: AssignDecl,
        pub cond: Expression,
        pub step: AssignDecl,
        pub body: Vec<Statement>
    }
    // TODO: add more statement types
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
pub struct Bind {   // binds a parameter to an argument or a port to a wire
    pub name: Identifier,
    pub value: Expression
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expression {
    Identifier(Identifier),
    ConstantInt(i64),
    BinaryBitOperation(Box<Expression>, BinBitOp, Box<Expression>),
    UnaryBitOperation(UnBitOp, Box<Expression>),
    BinaryArithmeticOperation(Box<Expression>, BinArithOp, Box<Expression>),
    UnaryArithmeticOperation(UnArithOp, Box<Expression>),
    Slice(Box<Expression>, Range)
    // TODO: add more expression types, e.g. logic operations, concatenation, etc.
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BinBitOp {
    And,
    Or,
    Xor,
    Xnor
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnBitOp {
    Not
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BinArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnArithOp {
    Neg
}

// Implementations
impl Module {
    pub fn with_def(id: Identifier, params: Option<ParameterSignature>, ports: Vec<Port>, body: Vec<Statement>) -> Self {
        let mut params = params.unwrap_or(ParameterSignature(vec![]));
        let mut body = body;

        // Add ports to the body
        for port in ports {
            let port_decl = Statement::Wire(WireDecl {
                name: port.name,
                width: port.width,
                init: None,
                io: Some(port.io.unwrap_or(Direction::Input))
            });
            body.insert(0, port_decl);
        }

        Module {
            name: id,
            params: params.0,
            body
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
    pub io: Direction
}