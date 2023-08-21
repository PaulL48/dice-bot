use nom::{
    character::complete::{digit1, multispace0},
    combinator::map_res,
    error::ParseError,
    sequence::delimited,
    IResult, Parser,
};

pub fn p_u128(s: &str) -> IResult<&str, u128> {
    map_res(digit1, str::parse)(s)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
