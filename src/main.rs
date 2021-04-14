use anyhow::anyhow;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::fs::OpenOptions;
use std::io::Read;

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).ok_or(anyhow!("no path given"))?;

    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut md5 = Md5::new();
    md5.input(&buffer);
    let result = md5.result_str();
    println!("{}", result);
    Ok(())
}
