use anyhow::{Error, Result};
use lz4_flex::decompress_size_prepended;
use oozle::decompress;

pub fn decompress_oodle(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) -> Result<()> {
    match unsafe {
        decompress(
            &compressed_data[..compressed_len],
            &mut decompressed_data[..decompressed_len],
        )
    } {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::msg("Failed to decompress oodle data")),
    }
}

pub fn decompress_lz(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) -> Result<()> {
    let decompressed_data_buffer: Vec<u8> =
        decompress_size_prepended(&compressed_data[..compressed_len])?;
    decompressed_data[..decompressed_len].copy_from_slice(&decompressed_data_buffer);
    Ok(())
}
