use super::super::ast;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Parameter(String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ParameterExpression {
    Constant(i32),
    ParamRef(Parameter),
    Add(Box<ParameterExpression>, Box<ParameterExpression>),
    Sub(Box<ParameterExpression>, Box<ParameterExpression>),
    Mul(Box<ParameterExpression>, Box<ParameterExpression>),
    Div(Box<ParameterExpression>, Box<ParameterExpression>),
    // we only support the above operations for now
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BitTensor {
    // Multi-dimensional tensor of bits
    pub shape: Vec<ParameterExpression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct View {
    // Polyhedral view of a tensor
    pub shape: Vec<ParameterExpression>,
    pub strides: Vec<Vec<i32>>, // including offsets
}

