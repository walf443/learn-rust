use crate::JSONValue;

pub fn parse_json_number<I: Iterator<Item = char>>(mut iter: I) -> anyhow::Result<JSONValue> {
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

#[cfg(test)]
mod tests {
    use crate::parse_json;

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
}

