use anyhow::Result;
use log::{debug, error};
use std::cell::RefCell;
use std::cmp::min_by;
use std::fs::File;
use std::io::{Read, Seek};
use std::rc::Rc;

use crate::toc::FileNode;
use crate::utils::compression::{decompress_lz, decompress_oodle};

pub fn decompress_post_ensmallening(
    entry: Rc<RefCell<FileNode>>,
    cache_reader: &mut File,
) -> Result<Vec<u8>> {
    cache_reader
        .seek(std::io::SeekFrom::Start(
            entry.borrow().cache_offset() as u64
        ))
        .unwrap();

    internal_decompress_post_ensmallening(
        entry.borrow().comp_len() as usize,
        entry.borrow().len() as usize,
        cache_reader,
    )
}

pub fn internal_decompress_post_ensmallening(
    compressed_len: usize,
    decompressed_len: usize,
    cache_reader: &mut File,
) -> Result<Vec<u8>> {
    let mut decompressed_data = vec![0u8; decompressed_len];
    let mut compressed_buffer = vec![0u8; 0x40000];
    let mut decompressed_pos = 0;

    while decompressed_pos < decompressed_len {
        let (mut block_compressed_len, mut block_decompressed_len) =
            get_block_lengths(cache_reader);
        if block_compressed_len == 0 && block_decompressed_len == 0 {
            block_compressed_len = compressed_len as usize;
            block_decompressed_len = decompressed_len as usize;
        }
        debug!(
            "Decompressing block, compressed_len: {}, decompressed_len: {}",
            block_compressed_len, block_decompressed_len
        );

        if decompressed_pos + block_decompressed_len > decompressed_len {
            error!(
                "Decompressed past the file length, decompressed_pos: {}, decompressed_len: {}, file_len: {}",
                decompressed_pos,
                block_decompressed_len,
                decompressed_len
            );
            panic!("Decompressed past the file length");
        }

        if block_compressed_len > min_by(get_file_length(cache_reader), 0x40000, |a, b| a.cmp(b)) {
            error!(
                "Tried to read beyond limits, probably not a compressed file, compressed_len: {}, file_len: {}",
                block_compressed_len,
                get_file_length(cache_reader)
            );
            panic!("Tried to read beyond limits, probably not a compressed file");
        }

        let is_oodle = is_oodle_block(cache_reader);
        cache_reader
            .read_exact(&mut compressed_buffer[..block_compressed_len])
            .unwrap();

        if is_oodle {
            debug!("Decompressing with oodle ({} bytes)", block_compressed_len);
            decompress_oodle(
                &compressed_buffer,
                block_compressed_len,
                &mut decompressed_data[decompressed_pos as usize..],
                block_decompressed_len,
            )?;
        } else {
            debug!("Decompressing with lz4 ({} bytes)", block_compressed_len);
            decompress_lz(
                &compressed_buffer,
                block_compressed_len,
                &mut decompressed_data[decompressed_pos as usize..],
                block_decompressed_len,
            )?;
        }
        debug!("Decompressed {} bytes", block_decompressed_len);
        decompressed_pos += block_decompressed_len;
    }

    Ok(decompressed_data)
}

pub fn decompress_pre_ensmallening(
    entry: Rc<RefCell<FileNode>>,
    cache_reader: &mut File,
) -> Result<Vec<u8>> {
    cache_reader
        .seek(std::io::SeekFrom::Start(
            entry.borrow().cache_offset() as u64
        ))
        .unwrap();

    internal_decompress_pre_ensmallening(
        entry.borrow().comp_len() as usize,
        entry.borrow().len() as usize,
        cache_reader,
    )
}

pub fn internal_decompress_pre_ensmallening(
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

fn is_oodle_block(cache_reader: &mut File) -> bool {
    let mut check_magic = [0u8; 1];
    cache_reader.read_exact(&mut check_magic).unwrap();
    cache_reader.seek(std::io::SeekFrom::Current(-1)).unwrap();
    check_magic[0] == 0x8C
}

fn get_block_lengths(cache_reader: &mut File) -> (usize, usize) {
    let mut block_info = [0u8; 8];
    cache_reader.read_exact(&mut block_info).unwrap();

    if block_info[0] != 0x80 || (block_info[7] & 0x0F) != 0x1 {
        cache_reader.seek(std::io::SeekFrom::Current(-8)).unwrap();
        return (0, 0);
    }

    let num1 = ((block_info[0] as u32) << 24)
        | ((block_info[1] as u32) << 16)
        | ((block_info[2] as u32) << 8)
        | (block_info[3] as u32);
    let num2 = ((block_info[4] as u32) << 24)
        | ((block_info[5] as u32) << 16)
        | ((block_info[6] as u32) << 8)
        | (block_info[7] as u32);

    let block_comp_len = (num1 >> 2) & 0xFFFFFF;
    let block_decomp_len = (num2 >> 5) & 0xFFFFFF;

    (block_comp_len as usize, block_decomp_len as usize)
}

fn get_file_length(cache_reader: &File) -> usize {
    cache_reader.metadata().unwrap().len() as usize
}
