use serde::{Deserialize, Serialize};
use std::{io, process::exit};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SchemaEntry {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    minimum: serde_json::Value,
    maximum: serde_json::Value,
    is_required: bool,
    fields: Vec<SchemaEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestCase {
    schema_entries: Vec<SchemaEntry>,
    data: serde_json::Map<String, serde_json::Value>,
    url: String,
    endpoint_name: String,
    code: i32,
    excluded_validators: Vec<String>,
}

fn main() {
    let mut str_in = String::new();
    io::stdin().read_line(&mut str_in).unwrap();
    let item: TestCase = serde_json::from_str(&str_in).unwrap();
    if let Err(err) = validate_item(item) {
        eprint!("{}", err);
        exit(1);
    }
}

fn validate_item(item: TestCase) -> Result<(), String> {
    let _ = item;
    todo!("validate the test case and return Err() to fail the test case");
}

