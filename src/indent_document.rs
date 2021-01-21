use serde_json::{json, Value};

use nom::{
    alt, call, complete, count, delimited, eof, many0, many1, map, named, named_args, opt,
    pair, tag, take_until, terminated,
    character::complete::line_ending
};

named!(
    pub parse_indent_document<&str, Value>,
    terminated!(
        map!(opt!(call!(parse_value_array, 0)), |v| v
            .unwrap_or_default()),
        eof!()
    )
);

named_args!(
    parse_value_array(indent_level: usize)<&str, Value>,
    map!(many1!(
        complete!(terminated!(
            alt!(
                alt!(call!(parse_colon_pair, indent_level) | call!(parse_value_pair_array, indent_level)) |
                map!(
                    call!(parse_string, indent_level),
                    |s| (s, Value::Null)
                )
            ),
            many0!(line_ending)
        ))
    ),
    |mut s| {
        if s.iter().any(|m| m.1.is_null()) {
            json!(s.drain(..).map(|(k, v)| if v.is_null() {json!(k)} else {json!({k: v})}).collect::<Vec<_>>())
        } else {
            json!(s.drain(..).map(|(k, v)| (k.into(), v)).collect::<serde_json::Map<_, _>>())
        }
    })
);

named_args!(
    parse_colon_pair(indent_level: usize)<&str, (&str, Value)>,
    map!(
        delimited!(
            count!(parse_indent, indent_level),
            separated_pair!(
                take_till!(|ch| ch == '\n' || ch == ':'),
                pair!(tag!(":"), many1!(char!(' '))),
                take_till1!(|ch| ch == '\n' || ch == ':')
            ),
            line_ending
        ),
        |(k, v)| (k.into(), json!(v))
    )
);

named_args!(
    parse_value_pair_array(indent_level: usize)<&str, (&str, Value)>,
    pair!(
        call!(parse_string, indent_level),
        call!(parse_value_array, indent_level + 1)
    )
);

named_args!(
    parse_string(indent_level: usize)<&str, &str>,
    delimited!(
        count!(parse_indent, indent_level),
        terminated!(
            take_till!(|ch| ch == '\n' || ch == ':'),
            opt!(char!(':'))
        ),
        line_ending
    )
);

named!(
    parse_indent<&str, &str>,
    alt!(
        tag!("    ") |
        tag!("\t")
    )
);
