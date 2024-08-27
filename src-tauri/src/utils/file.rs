use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn read_file_to_bytes(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
