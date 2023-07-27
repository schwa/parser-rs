mod ast;

// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

use anyhow::{anyhow, Result};
use ast::{Expr, Operator, Value};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, error::*, multi::*,
    sequence::*, *,
};
use nom_recursive::{recursive_parser, RecursiveInfo};

type Span<'a> = nom_locate::LocatedSpan<&'a str, RecursiveInfo>;

#[allow(dead_code)]
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn decimal(input: Span) -> IResult<Span, i64> {
    map(digit1, |s: Span| s.parse::<i64>().unwrap())(input) // TODO: Unwrap
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
    let inner = alt((
        map(tag("=="), |_| Operator::Eq),
        map(tag("!="), |_| Operator::Ne),
        map(tag("<="), |_| Operator::Le),
        map(tag("<"), |_| Operator::Lt),
        map(tag(">="), |_| Operator::Ge),
        map(tag(">"), |_| Operator::Gt),
    ));
    return delimited(multispace0, inner, multispace0)(s);
}

fn value(s: Span) -> IResult<Span, Value> {
    return alt((
        map(tag("null"), |_| Value::Null),
        map(tag("true"), |_| Value::Bool(true)),
        map(tag("false"), |_| Value::Bool(false)),
        map(identifier, |s| Value::Variable(s.to_string())),
        map(quoted_string, |s| Value::Str(s.to_string())),
        map(decimal, |v| Value::Int(v)),
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
    let span = Span::new_extra(s, RecursiveInfo::new());
    let (remaining, expression) =
        expression(span).map_err(|e| anyhow!("Failed to parse input. {:?}", e))?;
    if !remaining.is_empty() {
        return Err(anyhow!(
            "Failed to consume all of input (remaining: \"{}\").",
            remaining
        ));
    }
    return Ok(expression);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Expr, Operator, Value, VariableLookup};
    use std::collections::HashMap;

    fn span(s: &str) -> Span {
        return Span::new_extra(s, RecursiveInfo::new());
    }

    fn test_bool(s: &str) -> bool {
        return bool::try_from(&parse(s).unwrap().evaluate_().unwrap()).unwrap();
    }

    fn test_string(s: &str) -> String {
        return String::try_from(&parse(s).unwrap().evaluate_().unwrap()).unwrap();
    }

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
            Expr::ron("BinaryExpr(Eq,Value(Variable(\"name\")),Value(Str(\"John\")))")
        );

        assert_eq!(
            expression(span("name=='John'")).unwrap().1,
            Expr::ron("BinaryExpr(Eq,Value(Variable(\"name\")),Value(Str(\"John\")))")
        );
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("name == 'John'").unwrap(),
            Expr::ron("BinaryExpr(Eq,Value(Variable(\"name\")),Value(Str(\"John\")))")
        );
    }

    #[test]
    fn single_values() {
        assert_eq!(test_bool("true"), true);
        assert_eq!(test_bool("false"), false);
        assert_eq!(test_string("'hello'"), "hello");
    }

    #[test]
    fn evaluation_tests() {
        assert_eq!(test_bool("true == true"), true);
        assert_eq!(test_bool("true == false"), false);
        assert_eq!(test_bool("'hello' == 200"), false);
        assert_eq!(test_bool("100 == 200"), false);
        assert_eq!(test_bool("100 != 200"), true);
        assert_eq!(test_bool("100 > 200"), false);
        assert_eq!(test_bool("100 < 200"), true);
        assert_eq!(test_bool("100 >= 200"), false);
        assert_eq!(test_bool("100 <= 200"), true);
    }

    #[test]
    fn complex_test() {
        let name = "John";
        let context = Context {
            variables: vec![("name".to_string(), Value::Str(name.to_string()))]
                .into_iter()
                .collect(),
        };
        let f = format!("name == '{}'", name);
        let ast = parse(&f).unwrap();
        let result = ast.evaluate(&context).unwrap();
        assert_eq!(bool::try_from(&result).unwrap(), true);
    }
}
