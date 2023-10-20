use nparse::KEVStrToJson;

#[cfg(target_os = "linux")]
fn main() {
    use std::{fs::File, io::Read};

    let path = "/etc/os-release.txt";
    let mut out = String::new();
    {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.kev_str_to_json();
    let result = result.unwrap();
    let result = result.as_object().unwrap();
    println!("{:#?}", result);
}

#[cfg(not(target_os = "linux"))]
fn main() {
    use std::{fs::File, io::Read};

    let path = "data/os-release.txt";
    let mut out = String::new();
    {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.kev_str_to_json();
    let result = result.unwrap();
    let result = result.as_object().unwrap();
    println!("{:#?}", result);
}
