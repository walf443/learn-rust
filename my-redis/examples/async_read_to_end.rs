use std::io;
use std::string::FromUtf8Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;
    let result = String::from_utf8(buffer);
    match result {
        Ok(content) => {
            println!("{}", content);
        }
        Err(_) => {
            eprintln!("file is not utf-8 encoded");
            return Ok(());
        }
    }

    Ok(())
}
