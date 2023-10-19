use std::io::{BufRead, BufReader};

use nom::{
    character::complete::{anychar, char},
    combinator::{eof, recognize},
    multi::{many0_count, many_till},
    sequence::pair,
    IResult,
};
use serde_json::{json, Value};
fn parse_line(i: &str) -> IResult<&str, (usize, &str)> {
    pair(many0_count(char(' ')), recognize(many_till(anychar, eof)))(i)
}

pub fn parse_multiline_kv_str(s: &str) -> Result<Value, String> {
    let mut obj = json!({});

    let mut keys = vec![];

    for line in BufReader::new(s.as_bytes()).lines().flatten() {
        if let Ok((_, (sp, content))) = parse_line(&line) {
            loop {
                match keys.last() {
                    Some(&(a, _)) if a >= sp => {
                        keys.pop().unwrap();
                    }
                    _ => break,
                }
            }

            let mut colon_index = 0;
            let mut content = content.to_string();

            for c in content.chars() {
                if c == ':' {
                    break;
                }
                colon_index += 1;
            }

            if colon_index < content.len() {
                let mut key = content.clone();
                content = key.split_off(colon_index + 1);
                keys.push((sp, key));
            }

            let mut splited_keys = keys.iter().flat_map(|x| x.1.split(':').next());

            let mut entry = obj
                .as_object_mut()
                .unwrap()
                .entry(splited_keys.next().unwrap());

            for key in splited_keys {
                let obj = entry.or_insert_with(|| json!({}));
                match obj {
                    Value::Array(ar) => {
                        if ar.first().map(|x| x.is_object()) != Some(true) {
                            ar.insert(0, json!({}));
                        }
                        entry = ar.first_mut().unwrap().as_object_mut().unwrap().entry(key);
                    }
                    Value::Object(map) => {
                        entry = map.entry(key);
                    }
                    Value::String(_) => {
                        *obj = json!([{}, obj]);
                        entry = obj
                            .as_array_mut()
                            .unwrap()
                            .first_mut()
                            .unwrap()
                            .as_object_mut()
                            .unwrap()
                            .entry(key);
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }

            let obj = entry.or_insert_with(|| Value::Null);

            match obj.clone() {
                Value::Array(mut ar) => {
                    ar.push(json!(content.trim()));
                }
                Value::Object(map) => {
                    *obj = json!([map, content.trim()]);
                }
                Value::String(st) => {
                    *obj = json!([st, content.trim()]);
                }
                Value::Null => {
                    *obj = json!(content.trim());
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }

    Ok(obj)
}
