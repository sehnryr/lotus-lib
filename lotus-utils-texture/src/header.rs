use std::cmp::max;

use anyhow::{Error, Result};
use ddsfile::{AlphaMode, D3D10ResourceDimension, Header10};
use ddsfile::{DxgiFormat, FourCC, Header, PixelFormatFlags};
use derivative::Derivative;

use crate::dds_format::DDSFormat;
use crate::raw_header::RawTextureHeader;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct TextureHeader {
    pub header: Header,
    pub header10: Option<Header10>,

    #[derivative(Debug = "ignore")]
    pub f_cache_image_count: u8,

    #[derivative(Debug = "ignore")]
    pub f_cache_image_offsets: Vec<u32>,

    size: usize,
}

impl TextureHeader {
    fn new(
        header: Header,
        header10: Option<Header10>,
        f_cache_image_count: u8,
        f_cache_image_offsets: Vec<u32>,
        size: usize,
    ) -> Self {
        Self {
            header,
            header10,
            f_cache_image_count,
            f_cache_image_offsets,
            size,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn new_uncompressed(
        width: u32,
        height: u32,
        bits_per_pixel: u32,
        f_cache_image_count: u8,
        f_cache_image_offsets: Vec<u32>,
        size: usize,
    ) -> Self {
        let mut header = Header::default();
        header.width = width;
        header.height = height;
        header.pitch = Some(width * bits_per_pixel >> 3);

        header.spf.fourcc = None;
        header.spf.flags.insert(PixelFormatFlags::ALPHA_PIXELS);
        header.spf.flags.insert(PixelFormatFlags::RGB);
        header.spf.rgb_bit_count = Some(32);
        header.spf.r_bit_mask = Some(0x00FF0000);
        header.spf.g_bit_mask = Some(0x0000FF00);
        header.spf.b_bit_mask = Some(0x000000FF);
        header.spf.a_bit_mask = Some(0xFF000000);

        Self::new(
            header,
            None,
            f_cache_image_count,
            f_cache_image_offsets,
            size,
        )
    }

    fn new_dx10_less(
        width: u32,
        height: u32,
        fourcc: FourCC,
        f_cache_image_count: u8,
        f_cache_image_offsets: Vec<u32>,
        size: usize,
    ) -> Self {
        let mut header = Header::default();
        header.width = width;
        header.height = height;

        header.spf.fourcc = Some(fourcc);
        header.spf.flags.insert(PixelFormatFlags::FOURCC);

        Self::new(
            header,
            None,
            f_cache_image_count,
            f_cache_image_offsets,
            size,
        )
    }

    fn try_new_dx10(
        width: u32,
        height: u32,
        dxgi_format: DxgiFormat,
        f_cache_image_count: u8,
        f_cache_image_offsets: Vec<u32>,
        size: usize,
    ) -> Result<Self> {
        let header = Header::new_dxgi(height, width, None, dxgi_format, None, None, None)?;
        let header10 = Header10::new(
            dxgi_format,
            false,
            D3D10ResourceDimension::Texture2D,
            1,
            AlphaMode::Unknown,
        );

        Ok(Self::new(
            header,
            Some(header10),
            f_cache_image_count,
            f_cache_image_offsets,
            size,
        ))
    }
}

impl TryFrom<&[u8]> for TextureHeader {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self> {
        let header = RawTextureHeader::try_from(data)?;

        // Calculate the width and height
        let width: u32;
        let height: u32;
        let width_ratio = header.width_ratio as u32;
        let height_ratio = header.height_ratio as u32;
        if header.width_ratio > header.height_ratio {
            width = header.max_side_length;
            height = header.max_side_length * height_ratio / width_ratio;
        } else {
            width = header.max_side_length * width_ratio / height_ratio;
            height = header.max_side_length;
        }

        // Calculate the size
        let dds_format = DDSFormat::try_from(header.dds_format)?;
        let bits_per_pixel: u32 = dds_format.into();
        let size = (max(1, width >> 2) * max(1, height >> 2) * bits_per_pixel) as usize;

        if dds_format == DDSFormat::Uncompressed {
            return Ok(TextureHeader::new_uncompressed(
                width,
                height,
                bits_per_pixel,
                header.f_cache_image_count,
                header.f_cache_image_offsets,
                size,
            ));
        }

        let fourcc: FourCC = dds_format.try_into()?;
        if fourcc == FourCC(FourCC::DX10) {
            let dxgi_format: DxgiFormat = dds_format.into();
            return TextureHeader::try_new_dx10(
                width,
                height,
                dxgi_format,
                header.f_cache_image_count,
                header.f_cache_image_offsets,
                size,
            );
        }

        Ok(TextureHeader::new_dx10_less(
            width,
            height,
            fourcc,
            header.f_cache_image_count,
            header.f_cache_image_offsets,
            size,
        ))
    }
}
