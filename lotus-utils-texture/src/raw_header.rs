use anyhow::Error;
use zerocopy::{LittleEndian, ByteOrder};

pub struct RawTextureHeader<'a> {
    pub hash: &'a [u8; 16],
    pub merged_file_count: u32,
    pub file_paths: Vec<&'a str>,
    pub arguments_length: u32,
    pub arguments: &'a str, // TODO: Parse the arguments
    pub file_type: u32,
    pub unknown1: u8,
    pub f_cache_image_count: u8,
    pub unknown2: u8,
    pub dds_format: u8,
    pub mip_map_count: u32,
    pub f_cache_image_offsets: Vec<u32>,
    pub width_ratio: u16,
    pub height_ratio: u16,
    pub b_cache_max_width: u16,
    pub b_cache_max_height: u16,
    pub max_side_length: u32,
    pub offset: usize,
}

impl<'a> TryFrom<&'a [u8]> for RawTextureHeader<'a> {
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

        let unknown1 = data[offset];
        let f_cache_image_count = data[offset + 1];
        let unknown2 = data[offset + 2];
        let dds_format = data[offset + 3];
        offset += 4;

        let mip_map_count = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        let mut f_cache_image_offsets = Vec::with_capacity(mip_map_count as usize);
        for _ in 0..mip_map_count {
            let f_cache_image_offset = LittleEndian::read_u32(&data[offset..offset + 4]);
            f_cache_image_offsets.push(f_cache_image_offset);
            offset += 4;
        }

        let width_ratio = LittleEndian::read_u16(&data[offset..offset + 2]);
        let height_ratio = LittleEndian::read_u16(&data[offset + 2..offset + 4]);
        offset += 4;

        let b_cache_max_width = LittleEndian::read_u16(&data[offset..offset + 2]);
        let b_cache_max_height = LittleEndian::read_u16(&data[offset + 2..offset + 4]);
        offset += 4;

        let max_side_length = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;

        Ok(RawTextureHeader {
            hash: hash.try_into()?,
            merged_file_count,
            file_paths,
            arguments_length,
            arguments,
            file_type,
            unknown1,
            f_cache_image_count,
            unknown2,
            dds_format,
            mip_map_count,
            f_cache_image_offsets,
            width_ratio,
            height_ratio,
            b_cache_max_width,
            b_cache_max_height,
            max_side_length,
            offset,
        })
    }
}
