use std::convert::TryFrom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::tag_no_case,
    sequence::tuple,
    IResult, combinator::{map_res, opt}, character::complete::{digit0, digit1}
};

// The roll command should support multiple dice over addition and subtraction
// ex. !roll 3d8 + 2d6 + 4 + 6
// multiplication is not supported
// batches are supported
// ex. !roll 2 d20 + 10 - 2 -> Rolls d20 + 10 - 2 twice
// drop lowest n is supported
// 

struct Roll {
    dice_count: u128,
    faces: u128,
}


pub struct RollCommand {
    batches: u128,
    expression: Vec<()>,
    drop: u128
}

// pub struct Roll {
//     dice_count: i128,
//     faces: u128,
// }

// fn p_roll(s: &str) -> IResult<&str, Roll> {

// }

fn p_u128(s: &str) -> IResult<&str, u128> {
    map_res(digit1, str::parse)(s)
}

fn roll_components(s: &str) -> IResult<&str, (Option<u128>, &str, u128)> {
    tuple((
        opt(p_u128),
        tag_no_case("d"),
        p_u128
    ))(s)
}

impl TryFrom<&str> for RollCommand {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut full_roll = tuple((
            p_u128,
            tag_no_case("d"),
            p_u128
        ));

        let mut default_1_roll = tuple((
            tag_no_case("d"),
            p_u128
        ));

        // syntax: [<batch>] <expression> [<drop>]
        // separators: <batch><ws><expr> +/- <expr> +/- ...<ws><drop>

        // If the first character after the first word is not a + or a - then it is the batch prefix
        let mut iter = value.split_whitespace();

        // technically this is also valid
        // !roll 2 d20+1+2

        // I think this makes it a lookahead 1 language since the batch prefix is only
        // know not to be a constant because it is not followed by an arithmetic operator

        // what would the lexical elements of the expression be?
        // integer
        // dice specifier: [n]dm
        // +/-

        todo!()
    }
}