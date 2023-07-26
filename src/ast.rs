#[cfg(test)]
use ron;
#[cfg(test)]
use serde::Deserialize;

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug, Deserialize))]

pub enum Operator {
    Eq,
}

#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(test, derive(Debug, Deserialize))]
pub enum Value {
    Null,
    Bool(bool),
    Str(String),
    Variable(String),
}

impl Value {
    pub fn evaluate(&self, lookup: &dyn VariableLookup) -> Value {
        match self {
            Value::Variable(name) => lookup.get_variable(name),
            _ => self.clone(),
        }
    }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug, Deserialize))]

pub enum Expr {
    BinaryExpr(Operator, Box<Expr>, Box<Expr>),
    Value(Value),
}

pub trait VariableLookup {
    fn get_variable(&self, name: &str) -> Value;
}

pub struct EmptyLookup {}

impl VariableLookup for EmptyLookup {
    fn get_variable(&self, _name: &str) -> Value {
        Value::Null
    }
}

impl Expr {
    pub fn evaluate(&self, lookup: &dyn VariableLookup) -> Value {
        match self {
            Expr::BinaryExpr(op, left, right) => {
                let left = left.evaluate(lookup);
                let right = right.evaluate(lookup);
                match op {
                    Operator::Eq => Value::Bool(left == right),
                }
            }
            Expr::Value(value) => value.evaluate(lookup),
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use std::collections::HashMap;

    impl Expr {
        pub fn ron(s: &str) -> Expr {
            return ron::from_str(s).unwrap();
        }
    }

    struct Context {
        variables: HashMap<String, Value>,
    }

    impl VariableLookup for Context {
        fn get_variable(&self, name: &str) -> Value {
            self.variables.get(name).unwrap().clone()
        }
    }

    #[test]
    fn ron_encoding_test() {
        let expr = Expr::BinaryExpr(
            Operator::Eq,
            Box::new(Expr::Value(Value::Str("John".to_string()))),
            Box::new(Expr::Value(Value::Variable("name".to_string()))),
        );
        let decoded = Expr::ron("BinaryExpr(Eq,Value(Str(\"John\")),Value(Variable(\"name\")))");
        assert_eq!(expr, decoded);
    }
}
