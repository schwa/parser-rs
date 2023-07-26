#![allow(unused_imports)]
#![allow(dead_code)]

use ast::{Expr, Operator, Value, VariableLookup};
use std::collections::HashMap;
pub mod ast;
use parser::parse;

fn main() {
    parse("name == 'John'").unwrap();
}
