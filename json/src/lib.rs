use anyhow::{anyhow, Result};
use std::iter::Map;

#[derive(Debug)]
pub enum JSONValue {
    Object(Box<Map<String, JSONValue>>),
    Array(Box<Vec<JSONValue>>),
    String(String),
    Number(f64),
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

impl TryInto<f64> for JSONValue {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<f64, Self::Error> {
        match self {
            JSONValue::Number(num) => Ok(num),
            _ => Err(anyhow::anyhow!("not number")),
        }
    }
}

impl TryInto<String> for JSONValue {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<String, Self::Error> {
        match self {
            JSONValue::String(str) => Ok(str),
            _ => Err(anyhow::anyhow!("not string")),
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
    let cursor = iter.peek().unwrap();
    match cursor {
        'n' => return parse_json_null(iter),
        't' => return parse_json_true(iter),
        'f' => return parse_json_false(iter),
        '"' => return parse_json_string(iter),
        '0'..='9' => return parse_json_number(iter),
        _ => {}
    }
    let ary = vec![];

    let ary = JSONValue::Array(Box::new(ary));
    Ok(ary)
}

fn parse_json_null<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    for i in ['n', 'u', 'l', 'l'] {
        let ch = iter.next().unwrap();
        if ch != i {
            return Err(anyhow!("unknown char: '{}', expected: '{}'", ch, i));
        }
    }

    Ok(JSONValue::Null)
}

fn parse_json_true<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    for i in ['t', 'r', 'u', 'e'] {
        let ch = iter.next().unwrap();
        if ch != i {
            return Err(anyhow!("unknown char: '{}', expected: '{}'", ch, i));
        }
    }

    Ok(JSONValue::Bool(true))
}

fn parse_json_false<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    for i in ['f', 'a', 'l', 's', 'e'] {
        let ch = iter.next().unwrap();
        if ch != i {
            return Err(anyhow!("unknown char: '{}', expected: '{}'", ch, i));
        }
    }

    Ok(JSONValue::Bool(false))
}

fn parse_json_number<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    let mut str = String::new();
    loop {
        let char = iter.next();

        match char {
            None => break,
            Some(char) => {
                str.push(char);
            }
        }
    }

    let num = str.parse()?;

    Ok(JSONValue::Number(num))
}

fn parse_json_string<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    let _ = iter.next().unwrap();
    let result: String = iter.take_while(|s| *s != '"').collect();
    Ok(JSONValue::String(result))
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
    let num: f64 = result.try_into()?;
    assert_eq!(num, 0.0);

    Ok(())
}

#[test]
fn test_parse_number_success2() -> anyhow::Result<()> {
    let result = parse_json("999")?;
    let num: f64 = result.try_into()?;
    assert_eq!(num, 999.0);

    Ok(())
}

#[test]
fn test_parse_number_float() -> anyhow::Result<()> {
    let result = parse_json("3.14")?;
    let num: f64 = result.try_into()?;
    assert_eq!(num, 3.14);

    Ok(())
}

#[test]
fn test_parse_string_success2() -> anyhow::Result<()> {
    let result = parse_json("\"hoge\"")?;
    let str: String = result.try_into()?;
    assert_eq!(str, "hoge");

    Ok(())
}
