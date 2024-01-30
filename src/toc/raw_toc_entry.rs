// This const is the size of a single TOC entry in bytes. It is used to
// determine the size of the buffer to read from the TOC file.
// Same as mem::size_of::<RawTocEntry>()
pub const TOC_ENTRY_SIZE: usize = 96;

#[repr(C)]
pub struct RawTocEntry {
    pub cache_offset: i64,
    pub timestamp: i64,
    pub comp_len: i32,
    pub len: i32,
    pub reserved: i32,
    pub parent_dir_index: i32,
    pub name: [u8; 64],
}

impl TryFrom<&[u8]> for RawTocEntry {
    type Error = ();

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != TOC_ENTRY_SIZE {
            return Err(());
        }

        Ok(Self {
            cache_offset: i64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            timestamp: i64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            comp_len: i32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            len: i32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            reserved: i32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            parent_dir_index: i32::from_le_bytes(bytes[28..32].try_into().unwrap()),
            name: bytes[32..96].try_into().unwrap(),
        })
    }
}
