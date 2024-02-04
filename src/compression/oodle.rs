use anyhow::{Error, Result};
use oodle_safe::decompress;

pub fn decompress_oodle(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) -> Result<()> {
    let input = &compressed_data[..compressed_len];
    let output = &mut decompressed_data[..decompressed_len];

    let result = decompress(input, output, None, None, None, None);

    match result {
        Err(_) => Err(Error::msg("Failed to decompress oodle data")),
        Ok(_) => Ok(()),
    }
}
