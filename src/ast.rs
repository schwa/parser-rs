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
    And,
    Or,
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
            _ => Err(anyhow!("Not a bool (got {:?})", value)),
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::Str(v) => Ok(v.clone()),
            _ => Err(anyhow!("Not a string (got {:?})", value)),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Int(v) => Ok(v),
            _ => Err(anyhow!("Not an (got {:?})", value)),
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

impl Expr {
    pub fn dump(&self, depth: usize) {
        match self {
            Expr::BinaryExpr(op, left, right) => {
                println!("{}{:?}", "  ".repeat(depth), op);
                left.dump(depth + 1);
                right.dump(depth + 1);
            }
            Expr::Value(v) => {
                println!("{}{:?}", "  ".repeat(depth), v);
            }
        }
    }
}

impl Value {
    pub fn unparse(&self) -> String {
        match self {
            Value::Bool(v) => return format!("{}", v),
            Value::Str(v) => return format!("\"{}\"", v),
            Value::Int(v) => return format!("{}", v),
            Value::Variable(v) => return format!("{}", v),
            Value::List(v) => {
                let mut s = String::new();
                s.push('[');
                for (i, v) in v.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&v.unparse());
                }
                s.push(']');
                return s;
            }
        }
    }
}

impl Expr {
    pub fn unparse(&self) -> String {
        match self {
            Expr::BinaryExpr(op, left, right) => {
                let left = left.unparse();
                let right = right.unparse();
                match op {
                    Operator::Eq => return format!("({} == {})", left, right),
                    Operator::Ne => return format!("({} != {})", left, right),
                    Operator::Lt => return format!("({} < {})", left, right),
                    Operator::Le => return format!("({} <= {})", left, right),
                    Operator::Gt => return format!("({} > {})", left, right),
                    Operator::Ge => return format!("({} >= {})", left, right),
                    Operator::Contains => return format!("({} contains {})", left, right),
                    Operator::And => return format!("({} and {})", left, right),
                    Operator::Or => return format!("({} or {})", left, right),
                }
            }
            Expr::Value(value) => value.unparse(),
        }
    }
}

// MARK: -

pub trait VariableLookup {
    fn get_variable(&self, name: &str) -> Result<Value>;
}

// MARK: -

impl Expr {
    pub fn evaluate<T>(&self, lookup: &T) -> Result<Value>
    where
        T: VariableLookup,
    {
        match self {
            Expr::BinaryExpr(op, left, right) => {
                let left = left.evaluate(lookup)?;
                let right = right.evaluate(lookup)?;
                match op {
                    Operator::Eq => return Ok(Value::Bool(left == right)),
                    Operator::Ne => Ok(Value::Bool(left != right)),
                    Operator::Lt => Ok(Value::Bool(left.try_lt(&right)?)),
                    Operator::Le => Ok(Value::Bool(left.try_le(&right)?)),
                    Operator::Gt => Ok(Value::Bool(left.try_gt(&right)?)),
                    Operator::Ge => Ok(Value::Bool(left.try_ge(&right)?)),
                    Operator::Contains => match (left, right) {
                        (Value::List(left), right) => {
                            return Ok(Value::Bool(left.contains(&right)));
                        }
                        (Value::Str(left), Value::Str(right)) => {
                            return Ok(Value::Bool(left.contains(&right)));
                        }
                        _ => Err(anyhow!("`contains` operator invalid {:?}", op)),
                    }, //_ => Err(anyhow!("Unsupported operator {:?}", op)),
                    Operator::And => {
                        let left = bool::try_from(&left)?;
                        let right = bool::try_from(&right)?;
                        return Ok(Value::Bool(left && right));
                    }
                    Operator::Or => {
                        let left = bool::try_from(&left)?;
                        let right = bool::try_from(&right)?;
                        return Ok(Value::Bool(left || right));
                    }
                }
            }
            Expr::Value(value) => value.evaluate(lookup),
        }
    }
}

// MARK: -

impl Value {
    fn try_lt(&self, other: &Self) -> Result<bool> {
        Ok(self
            .partial_cmp(other)
            .ok_or(anyhow!("Cannot compare {:?} and {:?}", self, other))?
            == std::cmp::Ordering::Less)
    }
    fn try_le(&self, other: &Self) -> Result<bool> {
        let r =
            self.partial_cmp(other)
                .ok_or(anyhow!("Cannot compare {:?} and {:?}", self, other))?;
        return Ok(r == std::cmp::Ordering::Less || r == std::cmp::Ordering::Equal);
    }
    fn try_gt(&self, other: &Self) -> Result<bool> {
        Ok(self
            .partial_cmp(other)
            .ok_or(anyhow!("Cannot compare {:?} and {:?}", self, other))?
            == std::cmp::Ordering::Greater)
    }
    fn try_ge(&self, other: &Self) -> Result<bool> {
        let r =
            self.partial_cmp(other)
                .ok_or(anyhow!("Cannot compare {:?} and {:?}", self, other))?;
        return Ok(r == std::cmp::Ordering::Greater || r == std::cmp::Ordering::Equal);
    }
}

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
