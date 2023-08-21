use nom::{
    branch::alt, character::complete::char, character::complete::multispace1, combinator::opt,
    multi::many0, sequence::tuple, IResult,
};

use crate::{
    common_parse::{p_u128, ws},
    roll::{parse_roll_expression, RollExpression},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    let (input, operator) = alt((char('+'), char('-')))(input)?;

    if operator == '-' {
        Ok((input, Operator::Subtract))
    } else {
        Ok((input, Operator::Add))
    }
}

#[derive(Debug)]
pub struct RollCommand {
    batches: u128,
    expressions: Vec<(Operator, RollExpression)>,
}

impl RollCommand {
    pub fn evaluate(&self) -> String {
        let mut batch_strings = Vec::new();

        for _ in 0..self.batches {
            let batch_results = self
                .expressions
                .iter()
                .copied()
                .map(|(op, expr)| (op, expr, expr.evaluate()))
                .collect::<Vec<_>>();
            let mut total = 0;
            let mut batch_string = "`".to_string();
            for (operator, expression, result) in batch_results {
                match operator {
                    Operator::Add => total += result.contribution(),
                    Operator::Subtract => total -= result.contribution(),
                }

                if matches!(expression, RollExpression::Roll(_)) {
                    batch_string.push_str(&result.to_string());
                }
            }
            batch_string.push_str(&format!("` Result: `{}`", total));
            batch_strings.push(batch_string);
        }

        batch_strings.iter().join("\n")
    }
}

fn parse_roll_expression_sequence(input: &str) -> IResult<&str, Vec<(Operator, RollExpression)>> {
    // The first roll expression must not have an operator
    let (input, first_roll_expression) = ws(parse_roll_expression)(input)?;

    let (input, mut rest_roll_expression) =
        many0(tuple((ws(parse_operator), ws(parse_roll_expression))))(input)?;

    // Implicitly the first roll expression is an add
    rest_roll_expression.insert(0, (Operator::Add, first_roll_expression));

    Ok((input, rest_roll_expression))
}

pub fn parse_roll_command(input: &str) -> IResult<&str, RollCommand> {
    let (input, (batch, roll_expressions)) = tuple((
        ws(opt(tuple((p_u128, multispace1)))),
        parse_roll_expression_sequence,
    ))(input)?;

    Ok((
        input,
        RollCommand {
            batches: batch.unwrap_or((1, "")).0,
            expressions: roll_expressions,
        },
    ))
}
