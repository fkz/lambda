use serde_json::{Value, json};

fn main() {
    // Read all files named new.json recursively in criterion folder
 
    let values = glob::glob("target/criterion/**/new/estimates.json").unwrap().map(| path | {
        let path = path.unwrap();
        let file = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);
        let v: Value = serde_json::from_reader(reader).unwrap();
        let point_estimate = &v["mean"]["point_estimate"];
        let standard_error = &v["mean"]["standard_error"];
        let path2 = path.parent().unwrap().join("benchmark.json");
        let file2 = std::fs::File::open(&path2).unwrap();
        let reader2 = std::io::BufReader::new(file2);
        let v2: Value = serde_json::from_reader(reader2).unwrap();
        let title = &v2["title"];

        json!({ "value": point_estimate, "range": standard_error, "name": title, "unit": "ns"})
    }).collect();

    let json_str = serde_json::to_string(&Value::Array(values)).unwrap();
    std::fs::write(std::env::args().nth(1).unwrap(), json_str).unwrap();
}
