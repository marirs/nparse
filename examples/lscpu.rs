use nparse::KVStrToJson;

#[cfg(target_os = "linux")]
fn main() {
    use std::process::Command;

    let cmd = "lscpu";
    let output = Command::new(cmd).output();
    let output = output.unwrap();
    let output = String::from_utf8_lossy(&output.stdout[..]).to_string();

    // Convert to json obj
    let result = output.kv_str_to_json();
    let result = result.unwrap();
    let result = result.as_object().unwrap();
    println!("{:#?}", result);
}

#[cfg(not(target_os = "linux"))]
fn main() {
    use std::{fs::File, io::Read};

    let path = "data/lscpu.txt";
    let mut out = String::new();
    {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.kv_str_to_json();
    let result = result.unwrap();
    let result = result.as_object().unwrap();
    println!("{:#?}", result);
}
