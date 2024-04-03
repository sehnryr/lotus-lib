use anyhow::Error;
use zerocopy::{ByteOrder, LittleEndian};

pub struct RawAudioHeader<'a> {
    pub hash: &'a [u8; 16],
    pub merged_file_count: u32,
    pub file_paths: Vec<&'a str>,
    pub arguments_length: u32,
    pub arguments: &'a str, // TODO: Parse the arguments
    pub file_type: u32,
    pub format_tag: u32,
    pub unknown1: u32,
    pub unknown2: &'a [u8; 24],
    pub samples_per_second: u32,
    pub bits_per_sample: u8,
    pub channels: u8,
    pub unknown3: u32,
    pub average_bytes_per_second: u32,
    pub block_align: u16,
    pub samples_per_block: u16,
    pub unknown4: &'a [u8; 12],
    pub size: u32,
}

impl<'a> TryFrom<&'a [u8]> for RawAudioHeader<'a> {
    type Error = Error;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let hash = &data[0..16];
        let merged_file_count = LittleEndian::read_u32(&data[16..20]);

        let mut offset = 20;
        let mut file_paths = Vec::with_capacity(merged_file_count as usize);
        for _ in 0..merged_file_count {
            let path_length = LittleEndian::read_u32(&data[offset..offset + 4]) as usize;
            let path = std::str::from_utf8(&data[offset + 4..offset + 4 + path_length])?;

            file_paths.push(path);

            offset += 4 + path_length;
        }

        let arguments_length = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let arguments = std::str::from_utf8(&data[offset..offset + arguments_length as usize])?;
        offset += arguments_length as usize;

        // If the arguments length is > 0, then we need to skip the null byte
        if arguments_length > 0 {
            offset += 1;
        }

        let file_type = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let format_tag = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let unknown1 = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let unknown2 = &data[offset..offset + 24];
        offset += 24;

        let samples_per_second = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let bits_per_sample = data[offset];
        let channels = data[offset + 1];
        offset += 2;

        let unknown3 = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let average_bytes_per_second = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let block_align = LittleEndian::read_u16(&data[offset..offset + 2]);
        offset += 2;

        let samples_per_block = LittleEndian::read_u16(&data[offset..offset + 2]);
        offset += 2;

        let unknown4 = &data[offset..offset + 12];
        offset += 12;

        let size = LittleEndian::read_u32(&data[offset..offset + 4]);

        Ok(RawAudioHeader {
            hash: hash.try_into()?,
            merged_file_count,
            file_paths,
            arguments_length,
            arguments,
            file_type,
            format_tag,
            unknown1,
            unknown2: unknown2.try_into()?,
            samples_per_second,
            bits_per_sample,
            channels,
            unknown3,
            average_bytes_per_second,
            block_align,
            samples_per_block,
            unknown4: unknown4.try_into()?,
            size,
        })
    }
}
