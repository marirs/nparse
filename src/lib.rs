mod indent_document;
mod kv_str_document;
mod multiline_kv_str_document;
mod dotted_tree_document;

use serde_json::Value;

pub trait IndentToJson {
    fn indent_to_json(&self) -> Result<Value, String>;
}

impl IndentToJson for String {
    fn indent_to_json(&self) -> Result<Value, String> {
        //! Convert a well Indented String to a Json Document
        //!
        //! ## Example usage
        //!
        //! ```
        //! use std::{fs::File, io::Read};
        //! use nparse::IndentToJson;
        //!
        //! fn main() {
        //!     let path = "data/dmidecode.txt";
        //!     let mut out = String::new();
        //!     {
        //!         let mut f = File::open(path).unwrap();
        //!         f.read_to_string(&mut out).unwrap();
        //!     }
        //!     let result = out.indent_to_json();
        //!     println!("{:#?}", result.unwrap());
        //! }
        //! ```
        let s = self.to_string() + "\n";  // adding a `\n` for safe parsing
        let doc = indent_document::parse_indent_string(&s);
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
        //!
        //! ## Example usage
        //!
        //! ```
        //! use std::{fs::File, io::Read};
        //! use nparse::KVStrToJson;
        //!
        //! fn main() {
        //!     let path = "data/lscpu.txt";
        //!     let mut out = String::new();
        //!     {
        //!         let mut f = File::open(path).unwrap();
        //!         f.read_to_string(&mut out).unwrap();
        //!     }
        //!     let result = out.kv_str_to_json();
        //!     println!("{:#?}", result.unwrap());
        //! }
        //! ```
        let doc = kv_str_document::parse_kv_str_string(&self);
        match doc {
            Ok(res) => Ok(res.1),
            Err(e) => Err(e.to_string())
        }
    }
}

pub trait MultilineKVStrToJson {
    fn multiline_kv_str_to_json(&self) -> Result<Value, String>;
}

impl MultilineKVStrToJson for String {
    fn multiline_kv_str_to_json(&self) -> Result<Value, String> {
        //! Convert a Key Value String into a Json Document
        //!
        //! ## Example usage
        //!
        //! ```
        //! use std::{fs::File, io::Read};
        //! use nparse::MultilineKVStrToJson;
        //!
        //! fn main() {
        //!     let path = "data/win-systeminfo.txt";
        //!     let mut out = String::new();
        //!     {
        //!         let mut f = File::open(path).unwrap();
        //!         f.read_to_string(&mut out).unwrap();
        //!     }
        //!     let result = out.multiline_kv_str_to_json();
        //!     println!("{:#?}", result.unwrap());
        //! }
        //! ```
        let doc = multiline_kv_str_document::parse_multiline_kv_str(&self);
        match doc {
            Ok(res) => Ok(res),
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
        //!
        //! ## Example usage
        //!
        //! ```
        //! use std::{fs::File, io::Read};
        //! use nparse::DottedTreeToJson;
        //!
        //! fn main() {
        //!     let path = "data/sysctl.txt";
        //!     let mut out = String::new();
        //!     {
        //!         let mut f = File::open(path).unwrap();
        //!         f.read_to_string(&mut out).unwrap();
        //!     }
        //!     let result = out.dotted_tree_to_json();
        //!     println!("{:#?}", result.unwrap());
        //! }
        //! ```
        dotted_tree_document::parse_doted_tree(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::Read
    };

    fn read(path: &str) -> String {
        let mut out = String::new();
        {
            let mut f = File::open(path).unwrap();
            f.read_to_string(&mut out).unwrap();
        }
        out
    }

    #[test]
    fn test_indent_document() {
        let out = read("data/indent_test.txt");
        let result = out.indent_to_json();
        assert!(result.is_ok());

        let v = result.unwrap();
        let v = v.as_array().unwrap().into_iter().nth(1).unwrap();
        let parent = v.get("parent");
        assert!(parent.is_some());

        let child = parent.unwrap().as_object();
        assert!(child.is_some());

        let grand_children = child.unwrap().get("child");
        assert!(grand_children.is_some());
        let children = grand_children.unwrap().as_array().unwrap();
        assert_eq!(children.len(), 2);

        let child_name = children.iter().nth(0);
        assert!(child_name.is_some());
        assert_eq!(child_name.unwrap(), "grandchild1");

    }

    #[test]
    fn test_dmidecode_indent_document() {
        let out = read("data/dmidecode.txt");
        let result = out.indent_to_json();
        assert!(result.is_ok());

        let v = result.unwrap();
        let v = v.as_object();
        assert!(v.is_some());

        let bios_info = v.unwrap().get("BIOS Information");
        assert!(bios_info.is_some());

        let vendor_info = bios_info.unwrap().get("Vendor");
        assert!(vendor_info.is_some());
        let val = vendor_info.unwrap().as_str().unwrap().trim();
        let excepted = "Phoenix Technologies LTD";
        assert_eq!(val, excepted);
    }

    #[test]
    fn test_lscpu_kv_document() {
        let out = read("data/lscpu.txt");
        let result = out.kv_str_to_json();
        assert!(result.is_ok());

        let result = result.unwrap();
        let result = result.as_object();
        assert!(result.is_some());

        let result = result.unwrap();

        let arch_val = result.get("Architecture");
        assert!(arch_val.is_some());
        assert_eq!(arch_val.unwrap(), "x86_64");

        let model_val = result.get("Model");
        assert!(model_val.is_some());
        assert_eq!(model_val.unwrap(), "142");
    }

    #[test]
    fn test_win_systeminfo_multiline_kv_document() {
        let out = read("data/win-systeminfo.txt");
        let result = out.multiline_kv_str_to_json();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_object());
        let result = result.as_object();
        assert!(result.is_some());

        let result = result.unwrap();
        let owner = result.get("Registered Owner").unwrap().as_str().unwrap();
        assert_eq!(owner, "test-user");

        let processor = result.get("Processor(s)").unwrap().as_array();
        assert!(processor.is_some());
        let processor = processor.unwrap();
        let processor = processor.iter().nth(0).unwrap().get("[01]").unwrap().as_str().unwrap();
        assert_eq!(processor, "x64 Family 6 Model 142 Stepping 10 GenuineIntel ~2808 Mhz");
    }

    #[test]
    fn test_sysctl_dotted_document() {
        let out = read("data/sysctl.txt");
        let result = out.dotted_tree_to_json();
        assert!(result.is_ok());

        let result = result.unwrap();
        let result = result.as_object();
        assert!(result.is_some());

        let result = result.unwrap();

        // get a random value from `user` key -> 2 levels deep
        let user_val = result.get("user");
        assert!(user_val.is_some());

        let user_sub_val = user_val.unwrap().as_object();
        assert!(user_sub_val.is_some());

        let user_sub_val = user_sub_val.unwrap().get("bc_base_max");
        assert!(user_sub_val.is_some());
        let val = user_sub_val.unwrap().as_str().unwrap();
        let excepted = "99";
        assert_eq!(val, excepted);

        // get a random value from `machdep` key -> 3 levels deep
        let machdep_val = result.get("machdep");
        assert!(machdep_val.is_some());

        let machdep_sub_val = machdep_val.unwrap().as_object();
        assert!(machdep_sub_val.is_some());

        let machdep_sub_val = machdep_sub_val.unwrap().get("cpu");
        assert!(machdep_sub_val.is_some());

        let machdep_sub_sub_val = machdep_sub_val.unwrap().as_object();
        assert!(machdep_sub_sub_val.is_some());

        let machdep_sub_sub_val = machdep_sub_sub_val.unwrap().get("vendor");
        assert!(machdep_sub_sub_val.is_some());

        let val = machdep_sub_sub_val.unwrap().as_str().unwrap().trim();
        let excepted = "GenuineIntel";
        assert_eq!(val, excepted);

    }
}