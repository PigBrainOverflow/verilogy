use std::collections::HashMap;
use super::{ast, verilog_grammar};

pub struct Parser {
    modules: HashMap<String, ast::Module>,
    lalrpop_parser: verilog_grammar::TranslationUnitParser
}

impl Parser {
    pub fn new() -> Self {
        Self{modules: HashMap::new(), lalrpop_parser: verilog_grammar::TranslationUnitParser::new()}
    }

    pub fn parse(&mut self, src: &str) -> Result<(), String> {
        let unit: ast::TranslationUnit = self.lalrpop_parser.parse(src).map_err(|e| format!("Parsing error: {}", e))?;
        for module in unit.modules {
            let name = &module.name.0;
            if self.modules.contains_key(name) {
                return Err(format!("Module {} is already defined", name));
            } else {
                self.modules.insert(name.clone(), module);
            }
        }
        Ok(())
    }

    pub fn modules(&self) -> &HashMap<String, ast::Module> {
        &self.modules
    }
}