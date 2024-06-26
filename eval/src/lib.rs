pub mod environment;
pub mod lace_lib;
pub mod object;

use std::{self, cell::RefCell, collections::HashMap, fs, rc::Rc};

use crate::{environment::Environment, object::Object};
use lace_lexer::{
    token::{kind::TokenKind, Token},
    Lexer,
};
use lace_parser::{
    ast::{
        nodes::{ConditionalOperator, HashLiteral, IdentNode, IndexAccess, PrimitiveNode},
        statement::{BlockStatement, Statement},
        Expression, Program,
    },
    Parser,
};
use object::{builtin::BuiltinFunction, function::Function};

pub struct Eval {
    environment: Rc<RefCell<Environment>>,
}

impl Default for Eval {
    fn default() -> Self {
        Self::new()
    }
}

impl Eval {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }
}

impl Eval {
    pub fn eval(&mut self, program: Program) -> Object {
        let mut obj = Object::Null;

        for statement in program.statements {
            obj = self.eval_statement(statement);
        }

        obj
    }

    fn eval_block(&mut self, block: BlockStatement) -> Object {
        let mut res = Object::Null;
        for statement in block.statements {
            res = self.eval_statement(statement);
            match res {
                Object::Error(_) | Object::Return(_) => return res,
                _ => (),
            }
        }

        res
    }

    fn eval_statement(&mut self, statement: Statement) -> Object {
        match statement {
            Statement::Assignment(st) => {
                let val = self.eval_expression(st.val);
                if val.errored() {
                    return val;
                }
                self.environment.borrow_mut().upsert(st.name.label, val);
                Object::Null
            }
            Statement::Return(ret) => {
                let return_val = self.eval_expression(ret.returnable);
                if return_val.errored() {
                    return_val
                } else {
                    Object::Return(Box::new(return_val))
                }
            }
            Statement::Expression(expr) => self.eval_expression(expr),
            Statement::Source(sourceable) => {
                let mut fpath = std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .expect("") // TODO provide some insight into what went wrong
                    .to_owned();
                fpath.push('/');
                fpath.push_str(sourceable.path.to_str().unwrap());
                fpath.push_str(".lace");
                let code = match fs::read_to_string(fpath.clone()) {
                    Ok(code) => code,
                    Err(err) => {
                        println!("{:?}", fpath);
                        eprintln!("{err}");
                        std::process::exit(1);
                    }
                };

                let lexer = Lexer::new(code);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                if !parser.errors.is_empty() {
                    parser.errors.iter().for_each(|e| {
                        println!("{}", e.emit_err());
                    })
                }
                self.eval(program)
            }
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Object {
        match expression {
            Expression::Identifier(ident) => self.eval_ident(ident),
            Expression::Primitive(primitive) => Self::eval_primitive(primitive),
            Expression::Unary(prefix) => {
                let right = self.eval_expression(*prefix.right_expr);
                Self::eval_prefix(&prefix.operator, &right)
            }
            Expression::Binary(infix) => {
                let (left, right) = (
                    self.eval_expression(*infix.left_expr),
                    self.eval_expression(*infix.right_expr),
                );

                Self::eval_infix(&infix.operator, left, right)
            }
            Expression::Conditional(conditional) => self.eval_conditional(conditional),
            Expression::FunctionDef(func) => {
                let params = func.params;
                let body = func.body;

                Object::Function(Function {
                    params,
                    body,
                    environment: Rc::clone(&self.environment),
                })
            }
            Expression::FunctionCall(fn_call) => {
                let function = self.eval_expression(*fn_call.function);
                if function.errored() {
                    return function;
                }

                let args = self.eval_expressions(fn_call.args);
                if args.len() == 1 && args[0].errored() {
                    return args[0].clone();
                }

                self.apply_func(function, args)
            }
            Expression::Array(arr) => {
                let elements = self.eval_expressions(arr.elements);
                if elements.len() == 1 && elements[0].errored() {
                    elements[0].clone()
                } else {
                    Object::Array(elements)
                }
            }
            Expression::ArrIndex(index_access) => self.eval_index_expr(index_access),
            Expression::HashMapLiteral(hmap) => self.eval_hashmap_expr(hmap),
        }
    }

    fn apply_func(&mut self, function: Object, args: Vec<Object>) -> Object {
        match function {
            Object::Function(func) => {
                let extended_env = Self::extended_func_env(&func, args);
                let curr_env = Rc::clone(&self.environment);
                self.environment = Rc::new(RefCell::new(extended_env));
                let eval_body = self.eval_block(func.body);
                self.environment = curr_env;
                eval_body
            }
            Object::Builtin(bfunc) => {
                let returned = bfunc.apply(args.clone());
                if let BuiltinFunction::Read = bfunc {
                    if let Object::Error(_) = returned {
                        return returned;
                    } else {
                        let var_name = match args[0].clone() {
                            Object::Str(label) => label,
                            _ => {
                                return Object::Error("Couldn't find variable".into());
                            }
                        };
                        self.environment.borrow_mut().upsert(var_name, returned);
                        return Object::Null;
                    }
                }
                returned
            }
            _ => Object::Error(format!("{} not found", function)),
        }
    }

    fn extended_func_env(function: &Function, args: Vec<Object>) -> Environment {
        let mut env = Environment::new_enclosed_env(Rc::clone(&function.environment));
        for (param, arg) in function.params.iter().zip(args) {
            env.upsert(param.label.clone(), arg);
        }

        env
    }

    fn eval_hashmap_expr(&mut self, h_pairs: HashLiteral) -> Object {
        let mut hmap = HashMap::new();

        for (key, val) in h_pairs.pairs {
            let key = self.eval_expression(key);
            if key.errored() {
                return key;
            }

            if !matches!(
                key,
                Object::Integer(_) | Object::Char(_) | Object::Str(_) | Object::Boolean(_)
            ) {
                return Object::Error(format!("Cannot hash a {}", key.kind()));
            }

            let val = self.eval_expression(val);
            if val.errored() {
                return val;
            }

            hmap.insert(key, val);
        }

        Object::HashLiteral(hmap)
    }

    fn eval_index_expr(&mut self, index_expr: IndexAccess) -> Object {
        let collection = self.eval_expression(*index_expr.arr);
        if collection.errored() {
            return collection;
        }

        let index = self.eval_expression(*index_expr.index);
        if index.errored() {
            return index;
        }

        match (&collection, &index) {
            (Object::Array(a), Object::Integer(i)) => {
                let l = a.len();
                if *i < 0 {
                    return Object::Error("Negative indexing isn't valid".into());
                } else if *i >= l as i64 {
                    return Object::Error(format!(
                        "Index {} out of bounds for an array of length {}",
                        i, l
                    ));
                }
                a[*i as usize].clone()
            }
            (Object::HashLiteral(h), _) => match h.get(&index) {
                Some(h) => h.clone(),
                None => Object::Null,
            },
            _ => Object::Error(format!(
                "Did not find value {} for {}",
                collection.kind(),
                index.kind()
            )),
        }
    }

    fn eval_conditional(&mut self, conditional: ConditionalOperator) -> Object {
        let condition = self.eval_expression(*conditional.cond);
        if let Object::Error(_) = condition {
            return condition;
        }

        if let Object::Boolean(true) = condition {
            self.eval_block(conditional.consequence)
        } else if let Some(alternative) = conditional.alternative {
            self.eval_block(alternative)
        } else {
            Object::Null
        }
    }

    fn eval_ident(&self, ident: IdentNode) -> Object {
        match self.environment.borrow_mut().get(&ident.label) {
            Some(id) => id,
            None => match BuiltinFunction::try_builtin(&ident.label) {
                Some(blt) => blt,
                None => Object::Error("Identifier not found".into()),
            },
        }
    }

    fn eval_expressions(&mut self, expressions: Vec<Expression>) -> Vec<Object> {
        let mut res = Vec::new();

        for expression in expressions {
            let eval_expr = self.eval_expression(expression);
            if eval_expr.errored() {
                return vec![eval_expr];
            }
            res.push(eval_expr);
        }

        res
    }

    fn eval_primitive(primitive: PrimitiveNode) -> Object {
        match primitive {
            PrimitiveNode::IntegerLiteral(i) => Object::Integer(i),
            PrimitiveNode::FloatLiteral(f) => Object::Float(f),
            PrimitiveNode::CharLiteral(c) => Object::Char(c),
            PrimitiveNode::StringLiteral(s) => Object::Str(s),
            PrimitiveNode::BooleanLiteral(b) => Object::Boolean(b),
        }
    }

    pub fn eval_prefix(operator: &Token, right: &Object) -> Object {
        match operator.kind {
            TokenKind::Bang => Self::eval_bang_expr(right),
            TokenKind::Minus => Self::eval_minus_expr(right),
            _ => Object::Error(format!("Invalid operator: {}", operator)),
        }
    }

    pub fn eval_bang_expr(right: &Object) -> Object {
        let truth_value: bool = if let Object::Boolean(b) = right {
            *b
        } else {
            !(matches!(right, Object::Null))
        };

        Object::Boolean(!truth_value)
    }

    pub fn eval_minus_expr(right: &Object) -> Object {
        match right {
            Object::Integer(i) => Object::Integer(-i),
            Object::Float(f) => Object::Float(-f),
            _ => Object::Error("Invalid datatype".into()),
        }
    }

    pub fn eval_infix(operator: &Token, left: Object, right: Object) -> Object {
        // if left.kind() != right.kind() {
        //     return Object::Error(format!(
        //         "{} and {} datatypes do not match",
        //         left.kind(),
        //         right.kind()
        //     ));
        // }

        match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => {
                Self::eval_integer_infix_expr(operator, x, y)
            }
            (Object::Float(x), Object::Float(y)) => Self::eval_float_infix_expr(operator, x, y),
            (Object::Boolean(x), Object::Boolean(y)) => Self::eval_bool_infix_expr(operator, x, y),
            (Object::Str(st), Object::Str(sr)) => Self::eval_str_infix_expr(operator, st, sr),
            (Object::Str(st), Object::Integer(i)) => {
                Self::eval_str_infix_expr(operator, st, i.to_string())
            }
            _ => Object::Error(format!(
                "Cannot perform {} operation on this datatype",
                operator
            )),
        }
    }

    pub fn eval_integer_infix_expr(operator: &Token, x: i64, y: i64) -> Object {
        match operator.kind {
            TokenKind::Plus => Object::Integer(x + y),
            TokenKind::Minus => Object::Integer(x - y),
            TokenKind::ForwardSlash => Object::Integer(x / y),
            TokenKind::Modulo => Object::Integer(x % y),
            TokenKind::Asterisk => Object::Integer(x * y),
            TokenKind::Equal => Object::Boolean(x == y),
            TokenKind::NotEqual => Object::Boolean(x != y),
            TokenKind::LessThan => Object::Boolean(x < y),
            TokenKind::GreaterThan => Object::Boolean(x > y),
            TokenKind::LessThanEqual => Object::Boolean(x <= y),
            TokenKind::GreaterThanEqual => Object::Boolean(x >= y),
            _ => {
                unreachable!("{}", format!("No infix for {}", operator))
            }
        }
    }

    pub fn eval_float_infix_expr(operator: &Token, x: f64, y: f64) -> Object {
        match operator.kind {
            TokenKind::Plus => Object::Float(x + y),
            TokenKind::Minus => Object::Float(x - y),
            TokenKind::ForwardSlash => Object::Float(x / y),
            TokenKind::Modulo => Object::Float(x % y),
            TokenKind::Asterisk => Object::Float(x * y),
            TokenKind::Equal => Object::Boolean(x == y),
            TokenKind::NotEqual => Object::Boolean(x != y),
            TokenKind::LessThan => Object::Boolean(x < y),
            TokenKind::GreaterThan => Object::Boolean(x > y),
            TokenKind::LessThanEqual => Object::Boolean(x <= y),
            TokenKind::GreaterThanEqual => Object::Boolean(x >= y),
            _ => {
                println!("{}", operator);
                unreachable!("No infix for float")
            }
        }
    }

    pub fn eval_bool_infix_expr(operator: &Token, left: bool, right: bool) -> Object {
        match operator.kind {
            TokenKind::Equal => Object::Boolean(left == right),
            TokenKind::NotEqual => Object::Boolean(left != right),
            TokenKind::And => Object::Boolean(left && right),
            TokenKind::Or => Object::Boolean(left || right),
            _ => unreachable!("No infix for bool"),
        }
    }

    pub fn eval_str_infix_expr(operator: &Token, left: String, right: String) -> Object {
        match operator.kind {
            TokenKind::Equal => Object::Boolean(left == right),
            TokenKind::Plus => Object::Str(format!("{}{}", left, right)),
            TokenKind::Minus => {
                if left.ends_with(&right) {
                    Object::Str(
                        left.as_str()
                            .strip_suffix(right.as_str())
                            .unwrap()
                            .to_string(),
                    )
                } else {
                    Object::Error(format!("Cannot subtract {} from {}", right, left))
                }
            }
            _ => Object::Error(format!(
                "{} operation cannot be performed on strings",
                operator
            )),
        }
    }
}
