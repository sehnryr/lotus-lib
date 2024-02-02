use std::cmp::min_by;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use anyhow::Result;
use log::debug;

use crate::compression::lz::decompress_lz;
use crate::compression::oodle::decompress_oodle;

pub fn decompress_post_ensmallening(
    compressed_len: usize,
    decompressed_len: usize,
    cache_reader: &mut File,
) -> Result<Vec<u8>> {
    let mut decompressed_data = vec![0u8; decompressed_len];
    let mut compressed_buffer = vec![0u8; 0x40000];
    let mut decompressed_pos = 0;

    while decompressed_pos < decompressed_len {
        let (block_comp_len, block_decomp_len) =
            get_block_lengths(cache_reader)?.unwrap_or((compressed_len, decompressed_len));
        debug!(
            "Decompressing block, compressed_len: {}, decompressed_len: {}",
            block_comp_len, block_decomp_len
        );

        if decompressed_pos + block_decomp_len > decompressed_len {
            return Err(anyhow::anyhow!(
                "Decompressed past the file length, decompressed_pos: {}, decompressed_len: {}, file_len: {}",
                decompressed_pos,
                block_decomp_len,
                decompressed_len
            ));
        }

        let remaining_len = get_remaining_length(cache_reader)?;
        if block_comp_len > min_by(remaining_len, 0x40000, |a, b| a.cmp(b)) {
            return Err(anyhow::anyhow!(
                "Tried to read beyond limits, probably not a compressed file, compressed_len: {}, remaining_len: {}",
                block_comp_len,
                remaining_len
            ));
        }

        let is_oodle = is_oodle_block(cache_reader)?;
        cache_reader.read_exact(&mut compressed_buffer[..block_comp_len])?;

        if is_oodle {
            debug!("Decompressing with oodle ({} bytes)", block_comp_len);
            decompress_oodle(
                &compressed_buffer,
                block_comp_len,
                &mut decompressed_data[decompressed_pos as usize..],
                block_decomp_len,
            )?;
        } else if block_comp_len == block_decomp_len {
            debug!("Copying ({} bytes)", block_comp_len);
            decompressed_data[decompressed_pos as usize..decompressed_pos + block_decomp_len]
                .copy_from_slice(&compressed_buffer[..block_comp_len]);
        } else {
            debug!("Decompressing with lz4 ({} bytes)", block_comp_len);
            decompress_lz(
                &compressed_buffer,
                block_comp_len,
                &mut decompressed_data[decompressed_pos as usize..],
                block_decomp_len,
            )?;
        }
        debug!("Decompressed {} bytes", block_decomp_len);
        decompressed_pos += block_decomp_len;
    }

    Ok(decompressed_data)
}

pub fn is_oodle_block(cache_reader: &mut File) -> Result<bool> {
    let mut check_magic = [0u8; 1];
    cache_reader.by_ref().read_exact(&mut check_magic)?;
    cache_reader.seek(SeekFrom::Current(-1))?;
    Ok(check_magic[0] == 0x8C)
}

pub fn get_block_lengths(cache_reader: &mut File) -> Result<Option<(usize, usize)>> {
    let mut block_info = [0u8; 8];
    cache_reader.read_exact(&mut block_info)?;

    if block_info[0] != 0x80 || (block_info[7] & 0x0F) != 0x1 {
        cache_reader.seek(SeekFrom::Current(-8))?;
        return Ok(None);
    }

    let num1 = u32::from_be_bytes([block_info[0], block_info[1], block_info[2], block_info[3]]);
    let num2 = u32::from_be_bytes([block_info[4], block_info[5], block_info[6], block_info[7]]);

    let block_comp_len = (num1 >> 2) & 0xFFFFFF;
    let block_decomp_len = (num2 >> 5) & 0xFFFFFF;

    Ok(Some((block_comp_len as usize, block_decomp_len as usize)))
}

fn get_remaining_length(cache_reader: &mut File) -> Result<usize> {
    Ok(cache_reader.metadata()?.len() as usize - cache_reader.seek(SeekFrom::Current(0))? as usize)
}
