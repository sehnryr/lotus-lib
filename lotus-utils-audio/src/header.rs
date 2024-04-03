use anyhow::{Error, Result};

use crate::compression_format::CompressionFormat;
use crate::ogg::{get_segment_table, OggPage};
use crate::opus::{OpusHead, OpusTags};
use crate::raw_header::RawAudioHeader;

#[derive(Debug)]
pub struct AudioHeader {
    pub format_tag: CompressionFormat,
    pub stream_serial_number: u32,
    pub samples_per_second: u32,
    pub bits_per_sample: u8,
    pub channels: u8,
    pub average_bytes_per_second: u32,
    pub block_align: u16,
    pub samples_per_block: u16,
    pub size: u32,
}

impl AudioHeader {
    pub fn to_wav_pcm(&self) -> Result<Vec<u8>> {
        let block_align = (self.channels * self.bits_per_sample) as u16 >> 3;
        let average_bytes_per_second = self.samples_per_second * block_align as u32;

        let mut data = Vec::with_capacity(44);

        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&(self.size + 32).to_le_bytes()); // Size of the file minus 12 bytes
        data.extend_from_slice(b"WAVE");
        data.extend_from_slice(b"fmt ");
        data.extend_from_slice(&16u32.to_le_bytes()); // Size of the format chunk
        data.extend_from_slice(&0x01u16.to_le_bytes()); // Format tag
        data.extend_from_slice(&(self.channels as u16).to_le_bytes()); // Channels
        data.extend_from_slice(&self.samples_per_second.to_le_bytes()); // Samples per second
        data.extend_from_slice(&average_bytes_per_second.to_le_bytes()); // Average bytes per second
        data.extend_from_slice(&block_align.to_le_bytes()); // Block align
        data.extend_from_slice(&(self.bits_per_sample as u16).to_le_bytes()); // Bits per sample
        data.extend_from_slice(b"data");
        data.extend_from_slice(&self.size.to_le_bytes()); // Size of the data chunk

        Ok(data)
    }

    pub fn to_wav_adpcm(&self) -> Result<Vec<u8>> {
        let mut data = Vec::with_capacity(78);

        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&(self.size + 66).to_le_bytes()); // Size of the file minus 12 bytes
        data.extend_from_slice(b"WAVE");
        data.extend_from_slice(b"fmt ");
        data.extend_from_slice(&50u32.to_le_bytes()); // Size of the format chunk
        data.extend_from_slice(&0x02u16.to_le_bytes()); // Format tag
        data.extend_from_slice(&(self.channels as u16).to_le_bytes()); // Channels
        data.extend_from_slice(&self.samples_per_second.to_le_bytes()); // Samples per second
        data.extend_from_slice(&self.average_bytes_per_second.to_le_bytes()); // Average bytes per second
        data.extend_from_slice(&self.block_align.to_le_bytes()); // Block align
        data.extend_from_slice(&(self.bits_per_sample as u16).to_le_bytes()); // Bits per sample
        data.extend_from_slice(&32u16.to_le_bytes()); // Size of the extension
        data.extend_from_slice(&self.samples_per_block.to_le_bytes()); // Samples per block
        data.extend_from_slice(&7u16.to_le_bytes()); // Number of coefficients
        for coefficient in [
            [256, 0],
            [512, -256],
            [0, 0],
            [192, 64],
            [240, 0],
            [460, -208],
            [392, -232],
        ]
        .iter()
        {
            data.extend_from_slice(&(coefficient[0] as i16).to_le_bytes()); // Coefficient 1
            data.extend_from_slice(&(coefficient[1] as i16).to_le_bytes()); // Coefficient 2
        }
        data.extend_from_slice(b"data");
        data.extend_from_slice(&self.size.to_le_bytes()); // Size of the data chunk

        Ok(data)
    }

    pub fn to_opus(self) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        // Opus header
        let opus_head = OpusHead::new(1, self.channels as u8, 312, self.samples_per_second, 0, 0);
        let segment_table = get_segment_table(&Into::<Vec<u8>>::into(opus_head.clone()), 255);
        let header_page = OggPage::new(
            0x02,
            0,
            self.stream_serial_number,
            0,
            segment_table.len() as u8,
            segment_table,
            Into::<Vec<u8>>::into(opus_head),
        );
        data.extend_from_slice(&Into::<Vec<u8>>::into(header_page));

        // Opus tags
        let opus_tags = OpusTags::new("Warframe".to_string(), vec!["ARTIST=Warframe".to_string()]);
        let segment_table = get_segment_table(&Into::<Vec<u8>>::into(opus_tags.clone()), 255);
        let tags_page = OggPage::new(
            0x00,
            0,
            self.stream_serial_number,
            1,
            segment_table.len() as u8,
            segment_table,
            Into::<Vec<u8>>::into(opus_tags),
        );
        data.extend_from_slice(&Into::<Vec<u8>>::into(tags_page));

        Ok(data)
    }
}

impl TryFrom<&[u8]> for AudioHeader {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let raw_header = RawAudioHeader::try_from(data)?;

        Ok(AudioHeader {
            format_tag: CompressionFormat::try_from(raw_header.format_tag)?,
            stream_serial_number: rand::random::<u32>(),
            samples_per_second: raw_header.samples_per_second,
            bits_per_sample: raw_header.bits_per_sample,
            channels: raw_header.channels,
            average_bytes_per_second: raw_header.average_bytes_per_second,
            block_align: raw_header.block_align,
            samples_per_block: raw_header.samples_per_block,
            size: raw_header.size,
        })
    }
}
