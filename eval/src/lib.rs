pub mod object;

use lace_lexer::token::Token;
use lace_parser::ast::{
    nodes::{ConditionalOperator, PrimitiveNode},
    statement::{BlockStatement, Statement},
    Expression, Program,
};
use object::Object;

pub struct Eval {}

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
            Statement::Let(_) => todo!(),
            Statement::Return(ret) => {
                let return_val = self.eval_expression(ret.returnable);
                if let Object::Error(_) = return_val {
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
            Expression::Identifier(_) => todo!(),
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
            Expression::FunctionDef(_) => todo!(),
            Expression::FunctionCall(_) => todo!(),
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
        if left.kind() != right.kind() {
            return Object::Error(format!(
                "{} and {} datatypes do not match",
                left.kind(),
                right.kind()
            ));
        }
        match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => {
                Self::eval_integer_infix_expr(operator, x, y)
            }
            (Object::Float(x), Object::Float(y)) => Self::eval_float_infix_expr(operator, x, y),
            (Object::Boolean(x), Object::Boolean(y)) => Self::eval_bool_infix_expr(operator, x, y),
            _ => Object::Error("Non-matching datatypes".into()),
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
            _ => unreachable!(),
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
            _ => unreachable!(),
        }
    }

    pub fn eval_bool_infix_expr(operator: &Token, left: bool, right: bool) -> Object {
        match operator {
            Token::Equal => Object::Boolean(left == right),
            Token::NotEqual => Object::Boolean(left != right),
            Token::And => Object::Boolean(left && right),
            Token::Or => Object::Boolean(left || right),
            _ => unreachable!(),
        }
    }
}
