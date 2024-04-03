use anyhow::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionFormat {
    PCM,
    ADPCM,
    Opus,
}

impl TryFrom<u32> for CompressionFormat {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(CompressionFormat::PCM),
            0x05 => Ok(CompressionFormat::ADPCM),
            0x07 => Ok(CompressionFormat::Opus),
            _ => Err(Error::msg("Unknown compression format")),
        }
    }
}