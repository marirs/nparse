mod indent_document;
mod kv_str_document;
mod dotted_tree_document;

use serde_json::Value;

pub trait IndentToJson {
    fn indent_to_json(&self) -> Result<Value, String>;
}

impl IndentToJson for String {
    fn indent_to_json(&self) -> Result<Value, String> {
        //! Convert a well Indented String to a Json Document
        let doc = indent_document::parse_indent_string(&self);
        match doc {
            Ok(res) => Ok(res.1),
            Err(e) => Err(e.to_string())
        }
    }
}

pub trait KVStrToJson {
    fn kv_str_to_json(&self) -> Result<Value, String>;
}

impl KVStrToJson for String {
    fn kv_str_to_json(&self) -> Result<Value, String> {
        //! Convert a Key Value String into a Json Document
        let doc = kv_str_document::parse_kv_str_string(&self);
        match doc {
            Ok(res) => Ok(res.1),
            Err(e) => Err(e.to_string())
        }
    }
}

pub trait DottedTreeToJson {
    fn dotted_tree_to_json(&self) -> Result<Value, String>;
}

impl DottedTreeToJson for String {
    fn dotted_tree_to_json(&self) -> Result<Value, String> {
        //! Convert a dotted tree String into a Json Document
        dotted_tree_document::parse_doted_tree(&self)
    }
}