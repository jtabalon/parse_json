use serde_json;
use std::fs;

fn main() {
    let path = "./src/sample.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("unable to parse json");
    println!("{}", res)
}
