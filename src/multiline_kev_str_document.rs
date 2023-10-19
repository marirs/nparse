use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{complete, map},
    multi::separated_list0,
    sequence::{pair, separated_pair},
    IResult,
};
use serde_json::{json, Value};

pub(crate) fn parse_kev_str_string(i: &str) -> IResult<&str, Value> {
    map(
        separated_list0(
            line_ending,
            complete(separated_pair(
                take_until("="),
                pair(tag("="), space0),
                not_line_ending,
            )),
        ),
        |mut v| {
            json!(v
                .drain(..)
                .map(|(k, v): (&str, &str)| (k.to_string(), json!(v.trim())))
                .collect::<serde_json::Map<_, _>>())
        },
    )(i)
}
