use anyhow::{bail, Result};
use std::iter::Map;

fn main() -> Result<()> {
    let ary = parse_json("[\"foo\", 2]")?;

    println!("result: {:?}", ary);
    Ok(())
}

#[derive(Debug)]
enum JSONValue {
    Object(Box<Map<String, JSONValue>>),
    Array(Box<Vec<JSONValue>>),
    String(String),
    Number(i64),
    Bool(bool),
    Null,
}

fn parse_json(input: &str) -> Result<JSONValue> {
    let str = JSONValue::String("foo".to_string());
    let num = JSONValue::Number(2);
    let ary = vec![str, num];

    let ary = JSONValue::Array(Box::new(ary));
    Ok(ary)
}
