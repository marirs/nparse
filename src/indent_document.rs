use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1},
    character::complete::{char, line_ending},
    combinator::{complete, eof, map, opt},
    multi::{count, many0, many1},
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};
use serde_json::{json, Value};

pub(crate) fn parse_indent_string(i: &str) -> IResult<&str, Value> {
    terminated(
        map(opt(parse_value_array(0)), |v| v.unwrap_or_default()),
        eof,
    )(i)
}

fn parse_value_array(indent_level: usize) -> impl Fn(&str) -> IResult<&str, Value> {
    move |i| {
        map(
            many1(complete(terminated(
                alt((
                    parse_colon_pair(indent_level),
                    parse_value_pair_array(indent_level),
                    map(parse_string(indent_level), |s| (s, Value::Null)),
                )),
                many0(line_ending),
            ))),
            |mut s| {
                if s.iter().any(|m| m.1.is_null()) {
                    json!(s
                        .drain(..)
                        .map(|(k, v)| if v.is_null() { json!(k) } else { json!({k: v}) })
                        .collect::<Vec<_>>())
                } else {
                    json!(s
                        .drain(..)
                        .map(|(k, v)| (k.into(), v))
                        .collect::<serde_json::Map<_, _>>())
                }
            },
        )(i)
    }
}

fn parse_colon_pair(indent_level: usize) -> impl Fn(&str) -> IResult<&str, (&str, Value)> {
    move |i| {
        map(
            delimited(
                count(parse_indent, indent_level),
                separated_pair(
                    take_till(|ch| ch == '\n' || ch == ':'),
                    pair(tag(":"), many1(char(' '))),
                    take_till1(|ch| ch == '\n' || ch == ':'),
                ),
                line_ending,
            ),
            |(k, v)| (k, json!(v)),
        )(i)
    }
}

fn parse_value_pair_array(indent_level: usize) -> impl Fn(&str) -> IResult<&str, (&str, Value)> {
    move |i| {
        pair(
            parse_string(indent_level),
            parse_value_array(indent_level + 1),
        )(i)
    }
}

fn parse_string(indent_level: usize) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i| {
        delimited(
            count(parse_indent, indent_level),
            terminated(take_till(|ch| ch == '\n' || ch == ':'), opt(char(':'))),
            line_ending,
        )(i)
    }
}

fn parse_indent(i: &str) -> IResult<&str, &str> {
    alt((tag("    "), tag("\t")))(i)
}
