use std::collections::HashMap;

use crate::parser::Object;

pub struct Eval {
    env: HashMap<String, Object>,
}

impl Eval {
    pub fn eval(&mut self, object: Object) {
        match object {
            Object::List(list) => {
                for exprs in list {
                    match exprs {
                        Object::List(list) => {
                            self.eval_list(&list);
                        }
                        _ => {
                            panic!("eval: unsupported object type");
                        }
                    }
                }
            }
            _ => panic!("eval: unsupported object type"),
        }
    }

    fn eval_list(&mut self, list: &[Object]) -> Object {
        match &list[0] {
            Object::Symbol(symbol) => match symbol.as_str() {
                "+" => self.eval_add(&list[1..]),
                "-" => self.eval_subtract(&list[1..]),
                "*" => self.eval_multiply(&list[1..]),
                "/" => self.eval_division(&list[1..]),
                "print" => self.eval_print(&list[1..]),
                "define" => self.eval_define(&list[1..]),
                "lambda" => self.eval_lambda(&list[1..]),
                e => self.eval_variable(e, &list[1..]),
            },
            Object::List(list) => self.eval_list(list),
            Object::Integer(e) => Object::Integer(*e),
            Object::Lambda(params, list) => self.eval_lambda_execution(params, list, &list[1..]),
            _ => Object::Void,
        }
    }

    fn eval_print(&mut self, list: &[Object]) -> Object {
        if list.len() != 1 {
            unreachable!("eval_print: invalid number of arguments, must be exact one");
        }

        match &list[0] {
            Object::Integer(integer) => println!("{}", integer),
            Object::String(string) => println!("{}", string),
            Object::Bool(boolean) => println!("{}", boolean),
            Object::Symbol(symbol) => {
                println!("{:?}", self.env.get(symbol).unwrap());
            }
            Object::List(list) => println!("{:?}", self.eval_list(list)),
            _ => unreachable!("eval_print: unsupported object type"),
        }

        Object::Void
    }

    /// An abstraction to retrieve all the integers inside a list
    /// The Object type MUST be a integer, if not, this function will
    /// throw an runtime type-error.
    fn get_list_integers(&mut self, list: &[Object]) -> Vec<Object> {
        list.iter()
            .map(|f| match f {
                Object::Integer(integer) => Object::Integer(*integer),
                Object::List(list) => {
                    let list_eval = self.eval_list(list);
                    match list_eval {
                        Object::Integer(integer) => Object::Integer(integer),
                        _ => unreachable!("get_list_integers: unsupported object type"),
                    }
                }
                Object::Symbol(a) => match self.env.get(a) {
                    Some(Object::Integer(integer)) => Object::Integer(*integer),
                    _ => unreachable!("get_list_integers: unsupported object type"),
                },
                _ => unreachable!("get_list_integers: unsupported object type"),
            })
            .collect()
    }

    fn eval_add(&mut self, list: &[Object]) -> Object {
        let mut sum = 0;

        self.get_list_integers(list).iter().for_each(|f| match f {
            Object::Integer(integer) => sum += integer,
            _ => unreachable!("function get_list_integers treats the type."),
        });

        Object::Integer(sum)
    }

    fn eval_subtract(&mut self, list: &[Object]) -> Object {
        let mut sum = 0;

        self.get_list_integers(list).iter().for_each(|f| match f {
            Object::Integer(integer) => sum -= integer,
            _ => unreachable!("function get_list_integers treats the type."),
        });

        Object::Integer(sum)
    }

    fn eval_multiply(&mut self, list: &[Object]) -> Object {
        let mut sum = 1;

        self.get_list_integers(list).iter().for_each(|f| match f {
            Object::Integer(integer) => sum *= integer,
            _ => unreachable!("function get_list_integers treats the type."),
        });

        Object::Integer(sum)
    }

    fn eval_division(&mut self, list: &[Object]) -> Object {
        todo!("eval_division: implement this function");
    }

    /// Modifies the environment by inserting a new symbol with its value
    /// If the variable was already declared, will be overwritten.
    ///
    /// The first argument must be a symbol, and the second argument must be a list
    ///
    /// Can throw a runtime error.
    fn eval_define(&mut self, list: &[Object]) -> Object {
        if list.len() != 2 {
            unreachable!("eval_define: invalid number of arguments, must be exact two");
        }

        // First argument must be a symbol
        // @TODO: Needs to check if the symbol is a private keyword.
        if let Object::Symbol(symbol) = &list[0] {
            match &list[1] {
                Object::List(list) => {
                    let list_eval = self.eval_list(list);
                    self.env.insert(symbol.clone(), list_eval);
                },
                Object::Integer(integer) => {
                    self.env.insert(symbol.clone(), Object::Integer(*integer));
                },
                Object::String(string) => {
                    self.env.insert(symbol.clone(), Object::String(string.clone()));
                },
                Object::Bool(boolean) => {
                    self.env.insert(symbol.clone(), Object::Bool(*boolean));
                },
                Object::Symbol(symbol) => {
                    self.env.insert(symbol.clone(), Object::Symbol(symbol.clone()));
                },
                _ => {
                    unreachable!("eval_define: unsupported object type at second argument");
                }
            }
        } else {
            unreachable!("eval_define: unsupported object type");
        }

        Object::Void
    }

    // Evaluates lambda declaration
    //
    // @TODO: Improve doc
    fn eval_lambda(&mut self, list: &[Object]) -> Object {
        // Lambda is defined as (lambda LIST LIST)
        // we must check if the first argument is a "pure list" (only symbols)
        // the second argument must be a list, just a regular list.

        if list.len() != 2 {
            panic!(
                "eval_lambda: invalid number of arguments, expected exactly 2, received {}",
                list.len()
            );
        }

        let mut args: Vec<String> = Vec::new();

        match &list[0] {
            Object::List(list) => {
                for expr in list {
                    match expr {
                        Object::Symbol(e) => {
                            args.push(e.to_string());
                        }
                        _ => panic!("eval_lambda: invalid argument, must be a symbol"),
                    }
                }
            }
            _ => panic!("eval_lambda: first argument must be a pure-list."),
        }

        match &list[1] {
            Object::List(objects) => Object::Lambda(args, objects.to_vec()),
            _ => panic!("eval_lambda: second argument must be a list."),
        }
    }

    fn eval_lambda_execution(
        &mut self,
        params: &[String],
        list: &[Object],
        args: &[Object],
    ) -> Object {
        if params.len() != args.len() {
            panic!(
                "eval_lambda_execution: invalid number of arguments, expected {}, received {}",
                params.len(),
                args.len()
            );
        }

        let previous_scope = self.env.clone();

        for (param, arg) in params.iter().zip(args.iter()) {
            self.env.insert(param.clone(), arg.clone());
        }

        let result = self.eval_list(list);
        self.env = previous_scope;
        result
    }

    fn eval_variable(&mut self, variable: &str, list: &[Object]) -> Object {
        let obj = self.env.get(variable).cloned();

        if let Some(obj) = obj {
            match obj {
                Object::Lambda(params, paramslist) => {
                    self.eval_lambda_execution(&params, &paramslist, list)
                }
                _ => obj.clone(),
            }
        } else {
            panic!("eval_variable: variable {} not found", variable);
        }
    }
}

// constructor
impl Eval {
    pub fn new() -> Eval {
        Eval {
            env: HashMap::new(),
        }
    }
}
