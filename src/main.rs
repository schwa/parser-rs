use anyhow::{anyhow, Result};
use parser::ast::{Value, VariableLookup};
use parser::parse;
use std::collections::HashMap;

struct Context {
    variables: HashMap<String, Value>,
}

impl VariableLookup for Context {
    fn get_variable(&self, name: &str) -> Result<Value> {
        self.variables
            .get(name)
            .ok_or_else(|| anyhow!("Variable '{}' not found.", name))
            .map(|v| v.clone())
    }
}

fn main() {
    let name = "John";
    let context = Context {
        variables: vec![("name".to_string(), Value::Str(name.to_string()))]
            .into_iter()
            .collect(),
    };
    let f = format!("name == '{}'", name);
    let ast = parse(&f).unwrap();
    let result = ast.evaluate(&context).unwrap();
    println!("Result: {:?}", result);
}
