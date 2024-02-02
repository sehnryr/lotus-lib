use std::fs::File;
use std::io::Read;

use anyhow::Result;

use crate::compression::lz::decompress_lz;

pub fn decompress_pre_ensmallening(
    compressed_len: usize,
    decompressed_len: usize,
    cache_reader: &mut File,
) -> Result<Vec<u8>> {
    let mut compressed_data = vec![0u8; compressed_len];
    let mut decompressed_data = vec![0u8; decompressed_len];

    cache_reader.read_exact(&mut compressed_data).unwrap();

    decompress_lz(
        &compressed_data,
        compressed_len,
        &mut decompressed_data,
        decompressed_len,
    )?;

    Ok(decompressed_data)
}
