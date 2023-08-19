use nom::{IResult, combinator::map_res, character::complete::digit1};

pub fn p_u128(s: &str) -> IResult<&str, u128> {
    map_res(digit1, str::parse)(s)
}
