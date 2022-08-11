extern crate core;

use anyhow::{anyhow, Result};
use std::fmt::Error;
use std::iter::Map;

fn main() -> Result<()> {
    let ary = parse_json("[\"foo\", 2]")?;

    println!("result: {:?}", ary);
    Ok(())
}

#[derive(Debug)]
pub enum JSONValue {
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

pub fn parse_json(input: &str) -> Result<JSONValue> {
    let mut iter = input.chars().peekable();
    let cursor = iter.next().unwrap();
    match cursor {
        'n' => return parse_json_null(iter),
        't' => return parse_json_true(iter),
        'f' => return parse_json_false(iter),
        '"' => return parse_json_string(iter),
        _ => {}
    }
    let str = JSONValue::String("foo".to_string());
    let num = JSONValue::Number(2);
    let ary = vec![str, num];

    let ary = JSONValue::Array(Box::new(ary));
    Ok(ary)
}

fn parse_json_null<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    let char_u = iter.next().unwrap();
    if char_u != 'u' {
        return Err(anyhow!("unknown char: {}", char_u));
    }

    let char_l = iter.next().unwrap();
    if char_l != 'l' {
        return Err(anyhow!("unknown char: {}", char_l));
    }

    let char_u = iter.next().unwrap();
    if char_u != 'l' {
        return Err(anyhow!("unknown char: {}", char_u));
    }

    Ok(JSONValue::Null)
}

fn parse_json_true<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    let char_r = iter.next().unwrap();
    if char_r != 'r' {
        return Err(anyhow!("unknown char: {}", char_r));
    }

    let char_u = iter.next().unwrap();
    if char_u != 'u' {
        return Err(anyhow!("unknown char: {}", char_u));
    }

    let char_e = iter.next().unwrap();
    if char_e != 'e' {
        return Err(anyhow!("unknown char: {}", char_e));
    }

    Ok(JSONValue::Bool(true))
}

fn parse_json_false<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    let char_a = iter.next().unwrap();
    if char_a != 'a' {
        return Err(anyhow!("unknown char: {}", char_a));
    }

    let char_l = iter.next().unwrap();
    if char_l != 'l' {
        return Err(anyhow!("unknown char: {}", char_l));
    }

    let char_s = iter.next().unwrap();
    if char_s != 's' {
        return Err(anyhow!("unknown char: {}", char_s));
    }

    let char_e = iter.next().unwrap();
    if char_e != 'e' {
        return Err(anyhow!("unknown char: {}", char_e));
    }

    Ok(JSONValue::Bool(false))
}

fn parse_json_string<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    Ok(JSONValue::String("".to_string()))
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
