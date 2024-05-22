use std::{cell::RefCell, fmt::Display, rc::Rc};

use lace_parser::ast::{nodes::IdentNode, statement::BlockStatement};

use crate::environment::Environment;

#[derive(Clone)]
pub struct Function {
    pub params: Vec<IdentNode>,
    pub body: BlockStatement,
    pub environment: Rc<RefCell<Environment>>,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .params
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        write!(f, "fn({}) {{\n{}\n}}", params.join(", "), self.body)
    }
}
