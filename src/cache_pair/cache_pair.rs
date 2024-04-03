use std::path::PathBuf;

use anyhow::Result;

pub trait CachePair {
    const MAGIC_NUMBER: u64 = 0x1867C64E;
    const ARCHIVE_VERSION: u64 = 20;

    fn new(toc_path: PathBuf, cache_path: PathBuf, is_post_ensmallening: bool) -> Self;
    fn is_post_ensmallening(&self) -> bool;
    fn toc_path(&self) -> PathBuf;
    fn cache_path(&self) -> PathBuf;
    fn read_toc(&mut self) -> Result<()>;
    fn unread_toc(&mut self);
}
