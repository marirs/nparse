use nparse::MultilineKVStrToJson;

fn main() {
    use std::{fs::File, io::Read};

    let path = "data/win-systeminfo.txt";
    let mut out = String::new();
    {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.multiline_kv_str_to_json();
    let result = result.unwrap();
    let result = result.as_object().unwrap();
    println!("{:#?}", result);
}
