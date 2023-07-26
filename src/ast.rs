#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Eq,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq)]
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
mod tests {

    use super::*;
    use std::collections::HashMap;

    struct Context {
        variables: HashMap<String, Value>,
    }

    impl VariableLookup for Context {
        fn get_variable(&self, name: &str) -> Value {
            self.variables.get(name).unwrap().clone()
        }
    }

    #[test]
    fn basic_test() {
        let expr = Expr::BinaryExpr(
            Operator::Eq,
            Box::new(Expr::Value(Value::Str("John".to_string()))),
            Box::new(Expr::Value(Value::Variable("name".to_string()))),
        );

        let context = Context {
            variables: vec![("name".to_string(), Value::Str("John".to_string()))]
                .into_iter()
                .collect(),
        };

        let result = expr.evaluate(&context);
        assert!(result == Value::Bool(true))

        //        println!("{:?}", result)
    }
}
