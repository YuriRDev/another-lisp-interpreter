use crate::parser::{ArithmeticOp, BinaryOp, AST};

#[derive(Debug)]
pub enum Object {
    Boolean(bool),
    Number(i64),
    String(String),
    Void,
}

pub fn interpret(asts: Vec<AST>) {
    for ast in asts {
        evaluate(&ast);
    }
}

fn evaluate(ast: &AST) -> Object {
    match ast {
        AST::Number(e) => Object::Number(*e),
        AST::Boolean(e) => Object::Boolean(*e),
        AST::Print(children) => {
            println!("→ {:?}", print(evaluate(children)));
            Object::Void
        }

        AST::If(condition, _true, _false) => {
            if let Object::Boolean(a) = evaluate(condition) {
                if a {
                    evaluate(_true)
                } else {
                    evaluate(_false)
                }
            } else {
                unreachable!("Treated at the parser");
            }
        }

        AST::Binary(op, left, right) => {
            if let Object::Number(left) = &evaluate(left) {
                if let Object::Number(right) = &evaluate(right) {
                    return Object::Boolean(match op {
                        BinaryOp::Eq => left == right,
                        BinaryOp::Lt => left < right,
                        BinaryOp::Gt => left > right,
                    });
                }
            }
            unreachable!("Treated on parser");
        }

        AST::Arithmetic(op, list) => {
            let mut sum = 0;
            for c in list {
                if let Object::Number(r) = evaluate(c) {
                    match op {
                        ArithmeticOp::Minus => sum -= r,
                        ArithmeticOp::Plus => sum += r,
                    }
                }
            }
            Object::Number(sum)
        }
        AST::String(e) => Object::String(e.clone()),
        AST::Define(_x, _y) => Object::Void,
    }
}

fn print(obj: Object) -> String {
    match obj {
        Object::Number(e) => format!("{}", e),
        Object::String(e) => format!("\"{}\"", e),
        Object::Boolean(e) => format!("{}", e),
        Object::Void => "void".to_string(),
    }
}
