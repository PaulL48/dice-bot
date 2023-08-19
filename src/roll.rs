use std::convert::TryFrom;

use nom::{IResult, sequence::tuple, combinator::opt, bytes::complete::tag_no_case};

use crate::common_parse::p_u128;

pub enum RollExpression {
    Constant(i128),
    Roll(Roll),
}

impl TryFrom<&str> for RollExpression {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Parse the value as either a constant or a dice value
        if value.contains('d') {
            Ok(RollExpression::Roll(Roll::try_from(value)?))
        } else {
            Ok(RollExpression::Constant(value.parse::<i128>().map_err(|e| e.to_string())?))
        }
    }
}


pub struct Roll {
    dice_count: u128,
    faces: u128,
}

fn parse_roll(s: &str) -> IResult<&str, Roll> {
    let (input, (dice_count, _, faces)) = tuple((
        opt(p_u128),
        tag_no_case("d"),
        p_u128
    ))(s)?;

    Ok((input, Roll {
        dice_count: dice_count.unwrap_or(1), 
        faces
    }))
}

impl TryFrom<&str> for Roll {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // syntax: [<n>]d<m>
        let mut iter = value.split('d');
        let n = iter.next().ok_or(format!("Invalid dice specifier \"{}\": Malformed expression", value))?;
        let m = iter.next().ok_or(format!("Invalid dice specifier \"{}\": Malformed expression", value))?;

        let n = if let Ok(value) = n.parse() {
            value
        } else {
            1
        };

        // m is required
        let m = if let Ok(value) = m.parse() {
            value
        } else {
            return Err(format!("Invalid dice specifier \"{}\": Missing number of faces on die", m));
        };

        Ok(Roll {
            dice_count: n,
            faces: m,
        })
    }
}
