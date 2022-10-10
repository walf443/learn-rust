use anyhow::Result;
use json::parse_json;

fn main() -> Result<()> {
    let ary = parse_json("[\"foo\", 2]")?;

    println!("result: {:?}", ary);
    Ok(())
}
