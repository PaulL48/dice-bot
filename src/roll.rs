use std::fmt::{Display, Formatter};

use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::opt,
    sequence::tuple, IResult,
};

use crate::common_parse::p_u128;

use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Clone, Copy)]
pub enum RollExpression {
    Constant(u128),
    Roll(Roll),
}

impl RollExpression {
    pub fn evaluate(self) -> RollExpressionResult {
        match self {
            RollExpression::Constant(constant) => RollExpressionResult::new(vec![constant], None),
            RollExpression::Roll(roll) => {
                let range = Uniform::new(1, roll.faces + 1);
                let mut rng = rand::thread_rng();
                RollExpressionResult::new(
                    range
                        .sample_iter(&mut rng)
                        .take(roll.dice_count as usize)
                        .collect(),
                    roll.filter,
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RollExpressionResult {
    rolls: Vec<u128>,
    filter: Option<Filter>,
}

impl Display for RollExpressionResult {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.rolls.iter().join(", "))
    }
}

impl RollExpressionResult {
    fn new(rolls: Vec<u128>, filter: Option<Filter>) -> Self {
        Self { rolls, filter }
    }

    pub fn contribution(&self) -> u128 {
        let rolls = {
            let mut r = self.rolls.clone();
            r.sort();
            r
        };

        match self.filter {
            Some(Filter::Drop(value)) => rolls[value as usize..].iter().sum(),
            Some(Filter::Keep(value)) => rolls[rolls.len() - value as usize..].iter().sum(),
            None => rolls.iter().sum(),
        }
    }
}

pub fn parse_roll_expression(input: &str) -> IResult<&str, RollExpression> {
    alt((parse_roll, parse_const_roll_expression))(input)
}

#[derive(Debug, Clone, Copy)]
pub struct Roll {
    dice_count: u128,
    faces: u128,
    filter: Option<Filter>,
}

fn parse_const_roll_expression(input: &str) -> IResult<&str, RollExpression> {
    let (input, value) = p_u128(input)?;
    Ok((input, RollExpression::Constant(value)))
}

#[derive(Debug, Clone, Copy)]
enum Filter {
    Drop(u128),
    Keep(u128),
}

fn parse_drop_keep(input: &str) -> IResult<&str, Filter> {
    let (input, (tag, value)) = tuple((alt((tag_no_case("d"), tag_no_case("k"))), p_u128))(input)?;

    if tag == "d" || tag == "D" {
        Ok((input, Filter::Drop(value)))
    } else {
        Ok((input, Filter::Keep(value)))
    }
}

fn parse_roll(s: &str) -> IResult<&str, RollExpression> {
    let (input, (dice_count, _, faces, filter)) = tuple((
        opt(p_u128),
        tag_no_case("d"),
        p_u128,
        opt(tuple((multispace1, parse_drop_keep))),
    ))(s)?;

    Ok((
        input,
        RollExpression::Roll(Roll {
            dice_count: dice_count.unwrap_or(1),
            faces,
            filter: filter.map(|x| x.1),
        }),
    ))
}
