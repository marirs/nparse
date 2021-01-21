use serde_json::{json, Value};

use nom::{
    complete, named, map, take_until, pair,
    separated_list0, separated_pair, call, tag,
    character::complete::{line_ending, space1},
};

named!(
    pub(crate) parse_kv_str_string<&str, Value>,
    map!(
       separated_list0!(
            line_ending,
            complete!(separated_pair!(
                take_until!(":"),
                pair!(tag!(":"), call!(space1)),
                take_until!("\n")
            ))
        ),
        |mut v|
            json!(
                v.drain(..)
                    .map(|(k, v)| (k.to_string(), json!(v)))
                    .collect::<serde_json::Map<_, _>>()
            )
    )
);
