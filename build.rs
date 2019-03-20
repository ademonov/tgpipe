use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    std::fs::copy(".env", Path::new(&out_dir).join(".env"))?;

    Ok(())
}