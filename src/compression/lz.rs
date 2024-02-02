use anyhow::Result;
use lz4_flex::decompress_size_prepended;

pub fn decompress_lz(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) -> Result<()> {
    let decompressed_data_buffer = decompress_size_prepended(&compressed_data[..compressed_len])?;
    decompressed_data[..decompressed_len].copy_from_slice(&decompressed_data_buffer);
    Ok(())
}
