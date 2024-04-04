use anyhow::{Error, Result};
use bytebuffer::ByteBuffer;
use log::debug;
use lotus_lib::cache_pair::CachePairReader;
use lotus_lib::package::{Package, PackageType};
use lotus_lib::toc::{FileNode, Node};

use crate::compression_format::CompressionFormat;
use crate::header::AudioHeader;
use crate::kind::AudioKind;
use crate::ogg::{get_segment_table, OggPage};
use crate::raw_header::RawAudioHeader;

pub trait Audio {
    /// Checks if the given node is an audio file.
    ///
    /// # Errors
    ///
    /// Returns an error if the H cache is not found.
    fn is_audio(&self, node: &Node) -> Result<bool>;

    /// Decompresses the audio file data and get the name for the given node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to decompress the audio file for.
    ///
    /// # Returns
    ///
    /// A tuple containing the decompressed audio file data and the name of the audio file.
    fn decompress_audio(&self, node: &Node) -> Result<(Vec<u8>, String)>;
}

impl Audio for Package<CachePairReader> {
    fn is_audio(&self, node: &Node) -> Result<bool> {
        if !node.name().ends_with(".wav") {
            return Ok(false);
        }

        let h_cache = self
            .borrow(PackageType::H)
            .ok_or(Error::msg("No header file found"))?;

        let header_file_data = h_cache.decompress_data(node.clone())?;
        let header = match RawAudioHeader::try_from(header_file_data.as_slice()) {
            Ok(header) => header,
            Err(_) => return Ok(false),
        };

        match AudioKind::try_from(header.file_type) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn decompress_audio(&self, node: &Node) -> Result<(Vec<u8>, String)> {
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
        let header = AudioHeader::try_from(header_file_data.as_slice())?;
        debug!("Header: {:?}", header);

        match header.format_tag {
            CompressionFormat::PCM | CompressionFormat::ADPCM => {
                // Get the file data
                let f_cache = f_cache.unwrap();
                let b_cache = b_cache.unwrap();

                let f_file_node = f_cache.get_file_node(node.path());
                let b_file_node = b_cache.get_file_node(node.path());

                let mut buffer = ByteBuffer::new();

                if b_file_node.is_some() {
                    let b_file_node = b_file_node.unwrap();

                    debug!("Part B file node found!");

                    debug!("Cache offset: {}", b_file_node.cache_offset() as u64);
                    debug!("Cache audio size: {}", b_file_node.comp_len() as u64);
                    debug!("Decompressed audio size: {}", b_file_node.len() as u64);

                    let b_file_data = b_cache.decompress_data(b_file_node.clone())?;
                    buffer.write_bytes(&b_file_data);
                }

                if f_file_node.is_some() {
                    let f_file_node = f_file_node.unwrap();

                    debug!("Part F file node found!");

                    debug!("Cache offset: {}", f_file_node.cache_offset() as u64);
                    debug!("Cache audio size: {}", f_file_node.comp_len() as u64);
                    debug!("Decompressed audio size: {}", f_file_node.len() as u64);

                    let f_file_data = f_cache.decompress_data(f_file_node.clone())?;
                    buffer.write_bytes(&f_file_data);
                }

                debug!("Real audio size: {}", header.size as u64);

                let file_data = &buffer.as_bytes()[(buffer.len() - header.size as usize)..];

                let mut buffer = ByteBuffer::new();

                match header.format_tag {
                    CompressionFormat::PCM => buffer.write_bytes(&header.to_wav_pcm()?),
                    CompressionFormat::ADPCM => buffer.write_bytes(&header.to_wav_adpcm()?),
                    _ => return Err(Error::msg("Error extracting audio file")),
                }

                buffer.write_bytes(file_data);

                let file_name = {
                    let file_name = node.name();
                    let file_name = file_name.rsplit_once('.').unwrap_or((&file_name, "")).0;
                    format!("{}.wav", file_name)
                };

                Ok((buffer.as_bytes().to_vec(), file_name))
            }
            CompressionFormat::Opus => {
                // Get the file data
                let b_cache = b_cache.unwrap();
                let f_cache = f_cache.unwrap();

                let b_file_node = b_cache.get_file_node(node.path());
                let f_file_node = f_cache.get_file_node(node.path());

                let mut buffer = ByteBuffer::new();

                if f_file_node.is_some() {
                    let f_file_node = f_file_node.clone().unwrap();

                    debug!("Part F file node found!");

                    debug!("Cache offset: {}", f_file_node.cache_offset() as u64);
                    debug!("Cache audio size: {}", f_file_node.comp_len() as u64);
                    debug!("Decompressed audio size: {}", f_file_node.len() as u64);

                    let f_file_data = f_cache.decompress_data(f_file_node.clone())?;
                    buffer.write_bytes(&f_file_data);
                }

                if (f_file_node.is_none() || buffer.len() != header.size as usize)
                    && b_file_node.is_some()
                {
                    let b_file_node = b_file_node.unwrap();

                    debug!("Part B file node found!");

                    debug!("Cache offset: {}", b_file_node.cache_offset() as u64);
                    debug!("Cache audio size: {}", b_file_node.comp_len() as u64);
                    debug!("Decompressed audio size: {}", b_file_node.len() as u64);

                    let b_file_data = b_cache.decompress_data(b_file_node.clone())?;
                    buffer.write_bytes(&b_file_data);
                }

                debug!("Real audio size: {}", header.size as u64);

                let file_data = &buffer.as_bytes()[..header.size as usize];

                let stream_serial_number = header.stream_serial_number;
                let samples_per_second = header.samples_per_second as u64;
                let block_align = header.block_align;

                let mut buffer = ByteBuffer::new();

                buffer.write_bytes(&header.to_opus()?);

                // Write the opus data
                let mut page_sequence_number = 2;
                let mut granule_position = samples_per_second;

                let chunk_size = block_align as usize * 50;

                for chunk in file_data.chunks(chunk_size) {
                    let header_type = if chunk.len() < chunk_size { 0x04 } else { 0x00 };
                    let segment_table = get_segment_table(chunk, block_align.into());
                    let data_page = OggPage::new(
                        header_type,
                        granule_position,
                        stream_serial_number,
                        page_sequence_number,
                        segment_table.len() as u8,
                        segment_table,
                        chunk.to_vec(),
                    );

                    buffer.write_bytes(&Into::<Vec<u8>>::into(data_page));

                    page_sequence_number += 1;
                    granule_position += samples_per_second;
                }

                let file_name = {
                    let file_name = node.name();
                    let file_name = file_name.rsplit_once('.').unwrap_or((&file_name, "")).0;
                    format!("{}.opus", file_name)
                };

                Ok((buffer.as_bytes().to_vec(), file_name))
            }
        }
    }
}
