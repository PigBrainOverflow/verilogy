use std::rc::Rc;
use super::super::ast;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ParameterExpression {
    Constant(i32),
    Parameter(String),
    Add(Rc<ParameterExpression>, Rc<ParameterExpression>),
    Sub(Rc<ParameterExpression>, Rc<ParameterExpression>),
    Mul(Rc<ParameterExpression>, Rc<ParameterExpression>),
    Div(Rc<ParameterExpression>, Rc<ParameterExpression>),
    // we only support the above operations for now
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BitTensor {
    // Multi-dimensional tensor of bits
    pub shape: Vec<Rc<ParameterExpression>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct View {
    // Polyhedral view of a tensor
    pub shape: Vec<Rc<ParameterExpression>>,    // range of the original indices
    pub strides: Vec<Vec<i32>>, // including offsets
    pub tensor: Rc<BitTensor>, // the original tensor
}

pub struct Zip {
    // add to the last dimension
    pub left: Rc<BitTensor>,
    pub right: Rc<BitTensor>,
}