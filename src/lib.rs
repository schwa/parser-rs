#![allow(unused_imports)]
#![allow(dead_code)]

pub mod ast;

use anyhow::{anyhow, Context, Result};

use ast::{Expr, Operator, Value};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, error::*, multi::*,
    sequence::*, *,
};
use nom_locate::LocatedSpan;
use nom_recursive::{recursive_parser, RecursiveInfo};
use std::error::Error;
use std::fmt;

type Span<'a> = LocatedSpan<&'a str, RecursiveInfo>;

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn identifier(s: Span) -> IResult<Span, Span> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(s)
}

fn single_quoted_string(s: Span) -> IResult<Span, Span> {
    delimited(
        char('\''),
        escaped(is_not("\'"), '\\', one_of("\'n\\")),
        char('\''),
    )(s)
}

fn double_quoted_string(s: Span) -> IResult<Span, Span> {
    delimited(
        char('"'),
        escaped(is_not("\""), '\\', one_of("\"n\\")),
        char('"'),
    )(s)
}

fn quoted_string(s: Span) -> IResult<Span, Span> {
    alt((single_quoted_string, double_quoted_string))(s)
}

fn operator(s: Span) -> IResult<Span, Operator> {
    let inner = alt((map(tag("=="), |_| Operator::Eq),));
    return delimited(multispace0, inner, multispace0)(s);
}

fn value(s: Span) -> IResult<Span, Value> {
    return alt((
        map(identifier, |s| Value::Variable(s.to_string())),
        map(quoted_string, |s| Value::Str(s.to_string())),
    ))(s);
}

fn value_expression(s: Span) -> IResult<Span, Expr> {
    let (remaining, value) = value(s)?;
    return Ok((remaining, Expr::Value(value)));
}

#[recursive_parser]
fn binary_expression(s: Span) -> IResult<Span, Expr> {
    let (remaining, expression) = tuple((value_expression, operator, expression))(s)?;

    let (left, op, right) = expression;
    let expr = Expr::BinaryExpr(op, Box::new(left), Box::new(right));
    return Ok((remaining, expr));
}

#[recursive_parser]
fn expression(s: Span) -> IResult<Span, Expr> {
    return alt((binary_expression, value_expression))(s);
}

pub fn parse(s: &str) -> Result<Expr> {
    let span = LocatedSpan::new_extra(s, RecursiveInfo::new());
    let (remaining, expression) =
        expression(span).map_err(|_| anyhow!("Failed to parse input."))?;
    if !remaining.is_empty() {
        return Err(anyhow!("Failed to consume all of input."));
    }
    return Ok(expression);
}

fn span(s: &str) -> Span {
    return LocatedSpan::new_extra(s, RecursiveInfo::new());
}

#[cfg(test)]
mod tests {

    use super::*;
    use ast::{Expr, Operator, Value, VariableLookup};
    use std::collections::HashMap;

    #[test]
    fn basic_test() {
        assert_eq!(*identifier(span("input")).unwrap().1.fragment(), "input");
        assert_eq!(
            *quoted_string(span("'input'")).unwrap().1.fragment(),
            "input"
        );
        assert_eq!(
            *quoted_string(span("\"input\"")).unwrap().1.fragment(),
            "input"
        );
        assert_eq!(
            *quoted_string(span("'input'")).unwrap().1.fragment(),
            "input"
        );
        assert_eq!(operator(span("==")).unwrap().1, Operator::Eq);
        assert_eq!(operator(span(" ==")).unwrap().1, Operator::Eq);
        assert_eq!(operator(span("== ")).unwrap().1, Operator::Eq);
        assert_eq!(operator(span(" == ")).unwrap().1, Operator::Eq);
    }

    #[test]
    fn expressions_tests() {
        assert_eq!(
            binary_expression(span("name=='John'")).unwrap().1,
            Expr::BinaryExpr(
                Operator::Eq,
                Box::new(Expr::Value(Value::Variable("name".to_string()))),
                Box::new(Expr::Value(Value::Str("John".to_string()))),
            )
        );

        assert_eq!(
            expression(span("name=='John'")).unwrap().1,
            Expr::BinaryExpr(
                Operator::Eq,
                Box::new(Expr::Value(Value::Variable("name".to_string()))),
                Box::new(Expr::Value(Value::Str("John".to_string()))),
            )
        );
    }

    #[test]
    fn parse_test() {
        let expr = parse("name == 'John'").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryExpr(
                Operator::Eq,
                Box::new(Expr::Value(Value::Variable("name".to_string()))),
                Box::new(Expr::Value(Value::Str("John".to_string()))),
            )
        );
    }

    struct Context {
        variables: HashMap<String, Value>,
    }

    impl VariableLookup for Context {
        fn get_variable(&self, name: &str) -> Value {
            self.variables.get(name).unwrap().clone()
        }
    }

    // #[test]
    // fn complex_test() {
    //     let context = Context {
    //         variables: vec![("name".to_string(), Value::Str("John".to_string()))]
    //             .into_iter()
    //             .collect(),
    //     };

    //     let ast = parse("'x' == 'x'").unwrap();
    //     let result = ast.evaluate(&context);
    //     println!("{:?}", result);
    // }
}
