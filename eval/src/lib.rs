pub mod environment;
pub mod lace_lib;
pub mod object;

use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, object::Object};
use lace_lexer::token::Token;
use lace_parser::ast::{
    nodes::{ConditionalOperator, IdentNode, IndexAccess, PrimitiveNode},
    statement::{BlockStatement, Statement},
    Expression, Program,
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
            Statement::Let(st) => {
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
            Statement::Expr(expr) => self.eval_expression(expr),
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Object {
        match expression {
            Expression::Identifier(ident) => self.eval_ident(ident),
            Expression::Primitive(primitive) => Self::eval_primitive(primitive),
            Expression::Prefix(prefix) => {
                let right = self.eval_expression(*prefix.right_expr);
                Self::eval_prefix(&prefix.token, &right)
            }
            Expression::Infix(infix) => {
                let (left, right) = (
                    self.eval_expression(*infix.left_expr),
                    self.eval_expression(*infix.right_expr),
                );

                Self::eval_infix(&infix.token, left, right)
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

    fn eval_index_expr(&mut self, index_expr: IndexAccess) -> Object {
        let arr = self.eval_expression(*index_expr.arr);
        if arr.errored() {
            return arr;
        }

        let index = self.eval_expression(*index_expr.index);
        if index.errored() {
            return index;
        }

        match (arr, index) {
            (Object::Array(a), Object::Integer(i)) => {
                let l = a.len();
                if i < 0 {
                    return Object::Error("Negative indexing isn't valid".into());
                } else if i >= l as i64 {
                    return Object::Error(format!(
                        "Index {} out of bounds for an array of length {}",
                        i, l
                    ));
                }
                a[i as usize].clone()
            }
            _ => todo!(),
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
        match operator {
            Token::Bang => Self::eval_bang_expr(right),
            Token::Minus => Self::eval_minus_expr(right),
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
        match operator {
            Token::Plus => Object::Integer(x + y),
            Token::Minus => Object::Integer(x - y),
            Token::ForwardSlash => Object::Integer(x / y),
            Token::Modulo => Object::Integer(x % y),
            Token::Asterisk => Object::Integer(x * y),
            Token::Equal => Object::Boolean(x == y),
            Token::NotEqual => Object::Boolean(x != y),
            Token::LessThan => Object::Boolean(x < y),
            Token::GreaterThan => Object::Boolean(x > y),
            Token::LessThanEqual => Object::Boolean(x <= y),
            Token::GreaterThanEqual => Object::Boolean(x >= y),
            _ => {
                // println!("{}", operator);
                unreachable!("{}", format!("No infix for {}", operator))
            }
        }
    }

    pub fn eval_float_infix_expr(operator: &Token, x: f64, y: f64) -> Object {
        match operator {
            Token::Plus => Object::Float(x + y),
            Token::Minus => Object::Float(x - y),
            Token::ForwardSlash => Object::Float(x / y),
            Token::Modulo => Object::Float(x % y),
            Token::Asterisk => Object::Float(x * y),
            Token::Equal => Object::Boolean(x == y),
            Token::NotEqual => Object::Boolean(x != y),
            Token::LessThan => Object::Boolean(x < y),
            Token::GreaterThan => Object::Boolean(x > y),
            Token::LessThanEqual => Object::Boolean(x <= y),
            Token::GreaterThanEqual => Object::Boolean(x >= y),
            _ => {
                println!("{}", operator);
                unreachable!("No infix for float")
            }
        }
    }

    pub fn eval_bool_infix_expr(operator: &Token, left: bool, right: bool) -> Object {
        match operator {
            Token::Equal => Object::Boolean(left == right),
            Token::NotEqual => Object::Boolean(left != right),
            Token::And => Object::Boolean(left && right),
            Token::Or => Object::Boolean(left || right),
            _ => unreachable!("No infix for bool"),
        }
    }

    pub fn eval_str_infix_expr(operator: &Token, left: String, right: String) -> Object {
        match operator {
            Token::Plus => Object::Str(format!("{}{}", left, right)),
            Token::Minus => {
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
