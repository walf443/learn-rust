use anyhow::{bail, Result};
use std::fmt::Error;
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

impl JSONValue {
    pub fn is_null(&self) -> bool {
        match self {
            JSONValue::Null => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            JSONValue::Number(_num) => true,
            _ => false,
        }
    }
}

impl TryInto<i64> for JSONValue {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<i64, Self::Error> {
        match self {
            JSONValue::Number(num) => Ok(num),
            _ => Err(anyhow::anyhow!("not number")),
        }
    }
}

impl TryInto<bool> for JSONValue {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<bool, Self::Error> {
        match self {
            JSONValue::Bool(boolean) => Ok(boolean),
            _ => Err(anyhow::anyhow!("not boolean")),
        }
    }
}

fn parse_json(input: &str) -> Result<JSONValue> {
    if input == "null" {
        return Ok(JSONValue::Null);
    }
    if input == "true" {
        return Ok(JSONValue::Bool(true));
    }
    if input == "false" {
        return Ok(JSONValue::Bool(false));
    }
    let str = JSONValue::String("foo".to_string());
    let num = JSONValue::Number(2);
    let ary = vec![str, num];

    let ary = JSONValue::Array(Box::new(ary));
    Ok(ary)
}

#[test]
fn test_parse_null_success() -> anyhow::Result<()> {
    let result = parse_json("null")?;
    assert!(result.is_null());

    Ok(())
}

#[test]
fn test_parse_true_success() -> anyhow::Result<()> {
    let result = parse_json("true")?;
    let boolean: bool = result.try_into()?;
    assert_eq!(boolean, true);

    Ok(())
}

#[test]
fn test_parse_false_success() -> anyhow::Result<()> {
    let result = parse_json("false")?;
    let boolean: bool = result.try_into()?;
    assert_eq!(boolean, false);

    Ok(())
}

#[test]
fn test_parse_number_success() -> anyhow::Result<()> {
    let result = parse_json("0")?;
    let num: i64 = result.try_into()?;
    assert_eq!(num, 0);

    Ok(())
}
