pub mod ast;

use ast::{Expr, Operator, Value};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, error::*, multi::*,
    sequence::*, *,
};
use nom_locate::LocatedSpan;
use nom_recursive::{recursive_parser, RecursiveInfo};
use std::error::Error;
use std::fmt;

//type Span<'a> = LocatedSpan<&'a str, RecursiveInfo>;

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn single_quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(
        char('\''),
        escaped(is_not("\'"), '\\', one_of("\'n\\")),
        char('\''),
    )(input)
}

fn double_quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        escaped(is_not("\""), '\\', one_of("\"n\\")),
        char('"'),
    )(input)
}

fn quoted_string(input: &str) -> IResult<&str, &str> {
    alt((single_quoted_string, double_quoted_string))(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    ws(alt((map(tag("=="), |_| Operator::Eq),)))(input)

    // let mut eq = into(tag("=="));

    // let xxx: IResult<&str, Operator> = ws(alt((eq)))(input);

    // return xxx;
}

fn value(input: &str) -> IResult<&str, Value> {
    return alt((
        map(identifier, |s| Value::Variable(s.to_string())),
        map(quoted_string, |s| Value::Str(s.to_string())),
    ))(input);
}

//#[recursive_parser]
fn binary_expression(input: &str) -> IResult<&str, Expr> {
    let (remaining, expression) = tuple((expression, operator, expression))(&input)?;

    let (left, op, right) = expression;
    let expr = Expr::BinaryExpr(op, Box::new(left), Box::new(right));
    return Ok((remaining, expr));
}

//#[recursive_parser]
fn expression(input: &str) -> IResult<&str, Expr> {
    return alt((binary_expression, map(value, |v| Expr::Value(v))))(input);
}

#[derive(Debug)]
pub struct MyError {}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn parse(s: &str) -> Result<Expr, MyError> {
    let (remaining, expression) = expression(&s).map_err(|_| MyError {})?;
    // if !remaining.is_empty() {
    //     return Err(MyError {});
    // }
    println!("[{:?}]", remaining);
    println!("{:?}", expression);
    return Ok(expression);
}

#[cfg(test)]
mod tests {

    use super::*;
    use ast::{Expr, Operator, Value, VariableLookup};
    use std::collections::HashMap;

    #[test]
    fn basic_test() {
        assert_eq!(identifier("input").unwrap(), ("", "input"));
        assert_eq!(quoted_string("'input'").unwrap(), ("", "input"));
        assert_eq!(quoted_string("\"input\"").unwrap(), ("", "input"));
        assert_eq!(quoted_string("\"input\"").unwrap(), ("", "input"));
        assert_eq!(operator("==").unwrap(), ("", Operator::Eq));
        assert_eq!(operator("  ==").unwrap(), ("", Operator::Eq));
        assert_eq!(operator("==  ").unwrap(), ("", Operator::Eq));
        assert_eq!(operator("  ==  ").unwrap(), ("", Operator::Eq));
    }

    #[test]
    fn expressions_tests() {
        assert_eq!(
            binary_expression("name == 'John'").unwrap(),
            (
                "",
                Expr::BinaryExpr(
                    Operator::Eq,
                    Box::new(Expr::Value(Value::Variable("name".to_string()))),
                    Box::new(Expr::Value(Value::Str("John".to_string()))),
                )
            )
        );
        assert_eq!(
            expression("name == 'John'").unwrap(),
            (
                "",
                Expr::BinaryExpr(
                    Operator::Eq,
                    Box::new(Expr::Value(Value::Variable("name".to_string()))),
                    Box::new(Expr::Value(Value::Str("John".to_string()))),
                )
            )
        );
    }

    // #[test]
    // fn parse_test() {
    //     let expr = parse("name == 'John'").unwrap();
    //     println!("{:?}", expr);
    //     assert_eq!(
    //         expr,
    //         Expr::BinaryExpr(
    //             Operator::Eq,
    //             Box::new(Expr::Value(Value::Variable("name".to_string()))),
    //             Box::new(Expr::Value(Value::Str("John".to_string()))),
    //         )
    //     );
    // }

    struct Context {
        variables: HashMap<String, Value>,
    }

    impl VariableLookup for Context {
        fn get_variable(&self, name: &str) -> Value {
            self.variables.get(name).unwrap().clone()
        }
    }

    #[test]
    fn complex_test() {
        let context = Context {
            variables: vec![("name".to_string(), Value::Str("John".to_string()))]
                .into_iter()
                .collect(),
        };

        let ast = parse("'x' == 'x'").unwrap();
        let result = ast.evaluate(&context);
        println!("{:?}", result);
    }
}
