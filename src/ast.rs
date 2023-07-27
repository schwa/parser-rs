#[cfg(test)]
use ron;
#[cfg(test)]
use serde::Deserialize;

use anyhow::{anyhow, Result};

// MARK: -

#[derive(PartialEq, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub enum Operator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Contains,
    // In
    // And,
    // Or,
    // Not, // TODO
}

// MARK: -

#[derive(PartialEq, PartialOrd, Clone, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub enum Value {
    Bool(bool),
    Str(String),
    Int(i64),
    Variable(String),
    List(Vec<Value>),
}

impl Value {
    pub fn evaluate(&self, lookup: &dyn VariableLookup) -> Result<Value> {
        match self {
            Value::Variable(name) => lookup.get_variable(name),
            _ => Ok(self.clone()),
        }
    }
}

impl TryFrom<&Value> for bool {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::Bool(v) => Ok(v.clone()),
            _ => Err(anyhow!("Not a bool")),
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::Str(v) => Ok(v.clone()),
            _ => Err(anyhow!("Not a string")),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Int(v) => Ok(v),
            _ => Err(anyhow!("Not an int")),
        }
    }
}

// MARK: -

#[derive(PartialEq, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub enum Expr {
    BinaryExpr(Operator, Box<Expr>, Box<Expr>),
    Value(Value),
}

// MARK: -

pub trait VariableLookup {
    fn get_variable(&self, name: &str) -> Result<Value>;
}

// MARK: -

impl Expr {
    pub fn evaluate(&self, lookup: &dyn VariableLookup) -> Result<Value> {
        match self {
            Expr::BinaryExpr(op, left, right) => {
                let left = left.evaluate(lookup)?;
                let right = right.evaluate(lookup)?;
                match op {
                    Operator::Eq => Ok(Value::Bool(left == right)),
                    Operator::Ne => Ok(Value::Bool(left != right)),
                    Operator::Lt => Ok(Value::Bool(left < right)),
                    Operator::Le => Ok(Value::Bool(left <= right)),
                    Operator::Gt => Ok(Value::Bool(left > right)),
                    Operator::Ge => Ok(Value::Bool(left >= right)),
                    Operator::Contains => match (left, right) {
                        (Value::List(left), right) => {
                            return Ok(Value::Bool(left.contains(&right)));
                        }
                        (Value::Str(left), Value::Str(right)) => {
                            return Ok(Value::Bool(left.contains(&right)));
                        }
                        _ => Err(anyhow!("`contains` operator invalid {:?}", op)),
                    }, //_ => Err(anyhow!("Unsupported operator {:?}", op)),
                }
            }
            Expr::Value(value) => value.evaluate(lookup),
        }
    }
}

// MARK: -

pub struct EmptyLookup {}

impl VariableLookup for EmptyLookup {
    fn get_variable(&self, _name: &str) -> Result<Value> {
        Err(anyhow!("Variable not found."))
    }
}

impl Expr {
    pub fn evaluate_without_lookup(&self) -> Result<Value> {
        return self.evaluate(&EmptyLookup {});
    }
}

// MARK: -

#[cfg(test)]
pub mod tests {
    use super::*;

    impl Expr {
        pub fn ron(s: &str) -> Expr {
            return ron::from_str(s).unwrap();
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
