use std::mem;

use zerocopy::{FromBytes, FromZeroes};

// This const is the size of a single TOC entry in bytes. It is used to
// determine the size of the buffer to read from the TOC file.
pub(super) const TOC_ENTRY_SIZE: usize = mem::size_of::<TocEntry>();

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub(super) struct TocEntry {
    pub cache_offset: i64,
    pub timestamp: i64,
    pub comp_len: i32,
    pub len: i32,
    pub reserved: i32,
    pub parent_dir_index: i32,
    pub name: [u8; 64],
}
