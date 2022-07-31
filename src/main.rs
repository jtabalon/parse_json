use serde_json;
use serde_json::Value;
use std::fs;

fn parse_json(path: &str) -> Value {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("unable to parse json");
    // println!("{}", res)
    res
}

fn main() {
    let path = "./src/sample.json";
    let result = parse_json(path);
    println!("{:?}", result);
}
