use anyhow::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AudioKind {
    Audio139 = 0x8B, // TODO: Rename to something more descriptive
}

impl TryFrom<u32> for AudioKind {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x8B => Ok(AudioKind::Audio139),
            _ => Err(Error::msg("Unknown audio kind")),
        }
    }
}
