use nparse::DottedTreeToJson;

use std::process::Command;

fn main() {
    let cmd = "sysctl";
    let args = ["-a"];
    let output = Command::new(cmd).args(&args).output();
    let output = output.unwrap();
    let output = String::from_utf8_lossy(&output.stdout[..]).to_string();

    // Convert to json obj
    let result = output.dotted_tree_to_json();
    let result = result.unwrap();
    let result = result.as_object().unwrap();
    println!("{:#?}", result);
}
