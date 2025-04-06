use std::{collections::{HashMap, HashSet}, hash::Hash, i32, rc::Rc, vec};
use crate::ast::Statement;

use super::super::ast;

// an IR for parameterized netlist structures

#[derive(Debug, Clone, Eq)]
pub enum ParameterExpression {
    Constant(i32),
    Parameter(String),
    Add(Rc<ParameterExpression>, Rc<ParameterExpression>),
    Sub(Rc<ParameterExpression>, Rc<ParameterExpression>),
    Mul(Rc<ParameterExpression>, Rc<ParameterExpression>),
    Div(Rc<ParameterExpression>, Rc<ParameterExpression>),
    Mod(Rc<ParameterExpression>, Rc<ParameterExpression>),
    // we only support the above operations for now
}

impl PartialEq for ParameterExpression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParameterExpression::Constant(a), ParameterExpression::Constant(b)) => a == b,
            (ParameterExpression::Parameter(a), ParameterExpression::Parameter(b)) => a == b,
            (ParameterExpression::Add(a1, b1), ParameterExpression::Add(a2, b2)) => Rc::ptr_eq(a1, a2) && Rc::ptr_eq(b1, b2),
            (ParameterExpression::Sub(a1, b1), ParameterExpression::Sub(a2, b2)) => Rc::ptr_eq(a1, a2) && Rc::ptr_eq(b1, b2),
            (ParameterExpression::Mul(a1, b1), ParameterExpression::Mul(a2, b2)) => Rc::ptr_eq(a1, a2) && Rc::ptr_eq(b1, b2),
            (ParameterExpression::Div(a1, b1), ParameterExpression::Div(a2, b2)) => Rc::ptr_eq(a1, a2) && Rc::ptr_eq(b1, b2),
            (ParameterExpression::Mod(a1, b1), ParameterExpression::Mod(a2, b2)) => Rc::ptr_eq(a1, a2) && Rc::ptr_eq(b1, b2),
            _ => false,
        }
    }
}

impl std::hash::Hash for ParameterExpression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ParameterExpression::Constant(a) => a.hash(state),
            ParameterExpression::Parameter(a) => a.hash(state),
            ParameterExpression::Add(a1, b1) => {
                std::ptr::hash(a1.as_ref(), state);
                std::ptr::hash(b1.as_ref(), state);
            }
            ParameterExpression::Sub(a1, b1) => {
                std::ptr::hash(a1.as_ref(), state);
                std::ptr::hash(b1.as_ref(), state);
            }
            ParameterExpression::Mul(a1, b1) => {
                std::ptr::hash(a1.as_ref(), state);
                std::ptr::hash(b1.as_ref(), state);
            }
            ParameterExpression::Div(a1, b1) => {
                std::ptr::hash(a1.as_ref(), state);
                std::ptr::hash(b1.as_ref(), state);
            }
            ParameterExpression::Mod(a1, b1) => {
                std::ptr::hash(a1.as_ref(), state);
                std::ptr::hash(b1.as_ref(), state);
            }
        }
    }
}

pub struct BitTensor {
    // Multi-dimensional tensor of bits
    pub shape: Vec<Rc<ParameterExpression>>,
    pub from: Option<Rc<dyn Op>>,   // None if it is an input tensor
}

pub trait Op {}

pub struct Module {
    // Signature of the module
    pub name: String,
    pub params: HashMap<String, Rc<ParameterExpression>>,
    pub exprs: HashSet<Rc<ParameterExpression>>,
    pub tensors: HashSet<Rc<BitTensor>>,    // TODO: remove this
    pub inputs: HashMap<String, Rc<BitTensor>>,
    pub outputs: HashMap<String, Rc<BitTensor>>,
}

pub struct View {
    // Polyhedral view of a tensor
    pub shape: Vec<Rc<ParameterExpression>>,    // range of the original indices
    pub strides: Vec<Vec<i32>>, // including offsets
    pub input: Rc<BitTensor>, // the original tensor
}

pub struct Map {
    // we only support fixed instance whose parameters are known at compile time
    pub module: Rc<Module>,
    pub inputs: Vec<Rc<BitTensor>>, // inputs to the module
}

pub struct Apply {
    // connect an instance to the inputs
    pub module: Rc<Module>,
    pub inputs: Vec<Rc<BitTensor>>, // inputs to the module
}

pub struct Reduce {
    pub module: Rc<Module>,
    pub inputs: Vec<Rc<BitTensor>>, // inputs to the module
}

pub struct Zip {
    // zip multiple tensors by their last dimension
    // e.g. zip(<3, 5, 1>, <3, 5, 2>) -> <3, 5, 3>
    // zip(<3, 5, 1>, <3, 5, 2>, <3, 5, 3>) -> <3, 5, 6>
    // this means that all previous dimensions are the same
    inputs: Vec<Rc<BitTensor>>, // inputs to the module
}

impl Op for View {}
impl Op for Map {}
impl Op for Apply {}
impl Op for Reduce {}
impl Op for Zip {}

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            name: name.to_string(),
            params: HashMap::new(),
            exprs: HashSet::new(),
            tensors: HashSet::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    fn get_param_const(&mut self, value: i32) -> Rc<ParameterExpression> {
        // check if the value is already in the set
        let param = Rc::new(ParameterExpression::Constant(value));
        if self.exprs.contains(&param) {
            return param;
        }
        self.exprs.insert(param.clone());
        param
    }

    pub fn with_ast_module(ast_module: &ast::Module) -> Self {
        // gather parameters
        let mut module = Module::new(&ast_module.name.0);
        for param in &ast_module.params {
            module.params.insert(param.name.0.clone(), Rc::new(ParameterExpression::Parameter(param.name.0.clone())));
        }

        // gather ports
        for statement in &ast_module.body {
            module.extract_ports(statement);
        }

        module
    }

    fn extract_ports(&mut self, statement: &ast::Statement) {
        if let ast::Statement::Wire{name, width, init, io} = statement {
            if let Some(io) = io {  // input or output
                // convert width into dimensions
                let mut dims = vec![];
                if let Some(width) = width {

                }
                else {  // default to 1
                    dims.push(Rc::new(ParameterExpression::Constant(1)));
                }

            }
        }
    }
}