use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub verilog_grammar); // synthesized

pub mod ast;
pub mod parser;
pub mod preprocessor;
pub mod builder;