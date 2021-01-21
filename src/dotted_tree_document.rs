use serde_json::{json, Value};

use std::io::{BufRead, BufReader};

pub(crate) fn parse_doted_tree(s: &str) -> Result<Value, String> {
    let mut obj = json!({});

    for line in BufReader::new(s.as_bytes()).lines() {
        if let Ok(line) = line {
            let mut kv = line.splitn(2, ':');
            let keys = kv.next().unwrap();
            let v = json!(kv.next().unwrap());
            let mut splited_keys = keys.split('.');
            let mut entry = obj
                .as_object_mut()
                .unwrap()
                .entry(splited_keys.next().unwrap());
            for key in splited_keys {
                let obj = entry.or_insert_with(|| json!({}));
                entry = obj.as_object_mut().unwrap().entry(key);
            }
            match entry {
                serde_json::map::Entry::Vacant(a) => {
                    a.insert(v);
                }
                serde_json::map::Entry::Occupied(mut a) => {
                    a.insert(v);
                }
            }
        } else {
            return Err(String::from("Could not read string"))
        }
    }
    Ok(obj)
}