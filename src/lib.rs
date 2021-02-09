pub mod errors;

use std::error::Error;
use std::io::Read;

pub fn instantiate<B: AsRef<[u8]>>(
    buf: B,
) -> Result<(), Box<dyn Error>> {
    let mut magic_num = [0; 4];
    let mut reader = buf.as_ref();
    reader.read_exact(&mut magic_num)?;
    let magic_num = String::from_utf8(magic_num.to_vec())?;
    if magic_num != "\0asm" {
        return Err(errors::WasrusError::InvalidWasmFileError)?;
    }
    Ok(())
}
