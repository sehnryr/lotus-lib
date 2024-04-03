use anyhow::Error;
use ddsfile::{DxgiFormat, FourCC};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DDSFormat {
    BC1_UNORM,
    BC2_UNORM,
    BC3_UNORM,
    BC4_UNORM,
    BC5_UNORM,
    BC6H_UF16,
    BC7_UNORM,
    Uncompressed,
}

impl TryFrom<u8> for DDSFormat {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 | 0x01 => Ok(DDSFormat::BC1_UNORM),
            0x02 => Ok(DDSFormat::BC2_UNORM),
            0x03 => Ok(DDSFormat::BC3_UNORM),
            0x06 => Ok(DDSFormat::BC4_UNORM),
            0x07 => Ok(DDSFormat::BC5_UNORM),
            0x23 => Ok(DDSFormat::BC6H_UF16),
            0x22 => Ok(DDSFormat::BC7_UNORM),
            0x0A => Ok(DDSFormat::Uncompressed),
            _ => Err(Error::msg("Unknown DDS format")),
        }
    }
}

impl Into<u32> for DDSFormat {
    /// Returns the bits per pixel for the given DDS format.
    fn into(self) -> u32 {
        match self {
            DDSFormat::BC1_UNORM => 8,
            DDSFormat::BC2_UNORM => 16,
            DDSFormat::BC3_UNORM => 16,
            DDSFormat::BC4_UNORM => 8,
            DDSFormat::BC5_UNORM => 16,
            DDSFormat::BC6H_UF16 => 16,
            DDSFormat::BC7_UNORM => 16,
            DDSFormat::Uncompressed => 64,
        }
    }
}

impl Into<DxgiFormat> for DDSFormat {
    fn into(self) -> DxgiFormat {
        match self {
            DDSFormat::BC1_UNORM => DxgiFormat::BC1_UNorm,
            DDSFormat::BC2_UNORM => DxgiFormat::BC2_UNorm,
            DDSFormat::BC3_UNORM => DxgiFormat::BC3_UNorm,
            DDSFormat::BC4_UNORM => DxgiFormat::BC4_UNorm,
            DDSFormat::BC5_UNORM => DxgiFormat::BC5_UNorm,
            DDSFormat::BC6H_UF16 => DxgiFormat::BC6H_UF16,
            DDSFormat::BC7_UNORM => DxgiFormat::BC7_UNorm,
            DDSFormat::Uncompressed => DxgiFormat::R8G8B8A8_UNorm,
        }
    }
}

impl TryInto<FourCC> for DDSFormat {
    type Error = Error;

    fn try_into(self) -> Result<FourCC, Self::Error> {
        match self {
            DDSFormat::BC1_UNORM => Ok(FourCC(FourCC::DXT1)),
            DDSFormat::BC2_UNORM => Ok(FourCC(FourCC::DXT3)),
            DDSFormat::BC3_UNORM => Ok(FourCC(FourCC::DXT5)),
            DDSFormat::BC4_UNORM => Ok(FourCC(FourCC::ATI1)),
            DDSFormat::BC5_UNORM => Ok(FourCC(FourCC::ATI2)),
            DDSFormat::BC6H_UF16 => Ok(FourCC(FourCC::DX10)),
            DDSFormat::BC7_UNORM => Ok(FourCC(FourCC::DX10)),
            DDSFormat::Uncompressed => Err(Error::msg("Uncompressed format has no FourCC")),
        }
    }
}
