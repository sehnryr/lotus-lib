use std::fs::File;
use std::io::{Seek, SeekFrom};

use anyhow::{Error, Result};
use bytebuffer::ByteBuffer;
use log::debug;
use lotus_lib::cache_pair::{CachePair, CachePairReader};
use lotus_lib::compression::{decompress_post_ensmallening, get_block_lengths};
use lotus_lib::package::{Package, PackageType};
use lotus_lib::toc::{FileNode, Node};

use crate::header::TextureHeader;
use crate::raw_header::RawTextureHeader;
use crate::kind::TextureKind;

pub trait Texture {
    /// Check if the given node is a texture.
    ///
    /// # Errors
    ///
    /// Returns an error if the H cache is not found.
    fn is_texture(&self, node: &Node) -> Result<bool>;

    /// Decompresses the texture file data for the given node.
    ///
    /// # Returns
    ///
    /// A tuple containing the decompressed audio file data and the name of the audio file.
    fn decompress_texture(&self, node: &Node) -> Result<(Vec<u8>, String)>;
}

impl Texture for Package<CachePairReader> {
    fn is_texture(&self, node: &Node) -> Result<bool> {
        if !node.name().ends_with(".png") {
            return Ok(false);
        }

        let h_cache = self
            .borrow(PackageType::H)
            .ok_or(Error::msg("No header file found"))?;

        let header_file_data = h_cache.decompress_data(node.clone())?;
        let header = match RawTextureHeader::try_from(header_file_data.as_slice()) {
            Ok(header) => header,
            Err(_) => return Ok(false),
        };

        match TextureKind::try_from(header.file_type) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn decompress_texture(&self, node: &Node) -> Result<(Vec<u8>, String)> {
        let h_cache = self.borrow(PackageType::H);
        let f_cache = self.borrow(PackageType::F);
        let b_cache = self.borrow(PackageType::B);

        // Unwrap the H cache as there should always be a header file
        let h_cache = match h_cache {
            Some(h_cache) => h_cache,
            None => return Err(Error::msg("No header file found")),
        };

        // Get the decompressed header file data
        let header_file_data = h_cache.decompress_data(node.clone())?;

        // Parse the header file
        let header = TextureHeader::try_from(header_file_data.as_slice())?;
        debug!("Header: {:?}", header);

        let mut buffer = ByteBuffer::new();
        buffer.write_bytes(b"DDS ");
        header.header.write(&mut buffer)?;
        if let Some(header10) = &header.header10 {
            header10.write(&mut buffer)?;
        }

        if header.f_cache_image_count > 0 {
            let f_cache = match f_cache {
                Some(f_cache) => f_cache,
                None => return Err(Error::msg("No F cache found")),
            };
            let file_node = f_cache.get_file_node(node.path()).unwrap();

            debug!("Cache offset: {}", file_node.cache_offset() as u64);
            debug!("Cache image size: {}", file_node.comp_len() as u64);
            debug!("Real image size: {}", header.size() as u64);
            debug!("Decompressed image size: {}", file_node.len() as u64);

            let f_cache_offsets = &header.f_cache_image_offsets;

            if f_cache_offsets.len() != 0 {
                let cache_image_sub_offset = f_cache_offsets.last().unwrap_or(&0).to_owned();

                let mut f_cache_reader = File::open(f_cache.cache_path()).unwrap();
                let real_cache_image_sub_offset = get_real_cache_image_offset(
                    &mut f_cache_reader,
                    file_node.cache_offset() as usize,
                    cache_image_sub_offset as usize,
                )?;

                debug!("Cache image offset: {}", cache_image_sub_offset);
                debug!("Real cache image offset: {}", real_cache_image_sub_offset);

                f_cache_reader.seek(SeekFrom::Current(real_cache_image_sub_offset as i64))?;

                let file_data = decompress_post_ensmallening(
                    file_node.comp_len() as usize,
                    header.size() as usize,
                    &mut f_cache_reader,
                )?;
                buffer.write_bytes(&file_data);
            } else {
                // Fall back to the old method if the cache image offsets are not present
                let file_data = f_cache.decompress_data(file_node)?;
                buffer.write_bytes(&file_data[file_data.len() - header.size()..]);
            }
        } else {
            let b_cache = match b_cache {
                Some(b_cache) => b_cache,
                None => return Err(Error::msg("No B cache found")),
            };
            let file_node = b_cache.get_file_node(node.path()).unwrap();

            debug!("Cache offset: {}", file_node.cache_offset() as u64);
            debug!("Cache image size: {}", file_node.comp_len() as u64);
            debug!("Real image size: {}", header.size() as u64);
            debug!("Decompressed image size: {}", file_node.len() as u64);

            let file_data = b_cache.decompress_data(file_node)?;
            buffer.write_bytes(&file_data[file_data.len() - header.size()..]);
        }

        Ok((buffer.into_vec(), get_texture_file_name(node)))
    }
}

fn get_real_cache_image_offset(
    cache_reader: &mut File,
    cache_image_offset: usize,
    cache_image_sub_offset: usize,
) -> Result<usize> {
    cache_reader.seek(SeekFrom::Start(cache_image_offset as u64))?;

    const BLOCK_HEADER_LEN: usize = 8;

    let mut cache_offset_top: usize = 0;
    let mut cache_offset_bottom: usize = 0;

    loop {
        let (block_compressed_len, _) = get_block_lengths(cache_reader)?.unwrap_or((0, 0));
        cache_offset_top += block_compressed_len as usize + BLOCK_HEADER_LEN;

        if cache_offset_top >= cache_image_sub_offset {
            break;
        }

        cache_offset_bottom = cache_offset_top;
        cache_reader.seek(SeekFrom::Current(block_compressed_len as i64))?;
    }

    // Seek back to the start of the block
    cache_reader.seek(SeekFrom::Start(cache_image_offset as u64))?;

    let diff_top = cache_offset_top - cache_image_sub_offset;
    let diff_bottom = cache_image_sub_offset - cache_offset_bottom;

    if diff_top > diff_bottom {
        return Ok(cache_offset_bottom);
    } else {
        return Ok(cache_offset_top);
    }
}

fn get_texture_file_name(node: &Node) -> String {
    let mut file_name = node.name();
    if file_name.ends_with(".png") {
        file_name.truncate(file_name.len() - 4);
    }
    file_name.push_str(".dds");
    file_name
}
