use crate::JSONValue;
use anyhow::{anyhow, Result};

pub fn parse_json_true<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    for i in ['t', 'r', 'u', 'e'] {
        let ch = iter.next().unwrap();
        if ch != i {
            return Err(anyhow!("unknown char: '{}', expected: '{}'", ch, i));
        }
    }

    Ok(JSONValue::Bool(true))
}

pub fn parse_json_false<I: Iterator<Item = char>>(mut iter: I) -> Result<JSONValue> {
    for i in ['f', 'a', 'l', 's', 'e'] {
        let ch = iter.next().unwrap();
        if ch != i {
            return Err(anyhow!("unknown char: '{}', expected: '{}'", ch, i));
        }
    }

    Ok(JSONValue::Bool(false))
}

#[cfg(test)]
mod tests {
    use crate::parse_json;

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
}
