pub struct RawTocEntry {
    pub cache_offset: i64,
    pub timestamp: i64,
    pub comp_len: i32,
    pub len: i32,
    pub reserved: i32,
    pub parent_dir_index: i32,
    pub name: [u8; 64],
}
