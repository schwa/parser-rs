#![allow(unused_imports)]
#![allow(dead_code)]

use ast::{Expr, Operator, Value, VariableLookup};
use std::collections::HashMap;
pub mod ast;
use parser::parse;

fn main() {
    let x = parse("name == 'John'").unwrap();
    println!("{:?}", x);
}
