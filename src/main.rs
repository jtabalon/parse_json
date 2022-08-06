use serde_json;
use serde_json::Value;
use std::fs;
use std::fs::File;

fn parse_json(path: &str) -> Value {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("unable to parse json");
    res
}

fn main() {
    let path = "./src/sample.json";
    let result = parse_json(path);

    let mut f = File::create("foo.txt");

    // fs::write("foo.txt", &result)

    f.write_all(b"Hello, World!")


    // println!("{:?}", result);
    // println!("{:?}", result[0].len());
    // println!("{:?}", result[0]["Country"]);
    // let country_result = result.clone();
    // let country_name: Value = country_result[0]["Country"];
    // println!("{}", country_name);
}
