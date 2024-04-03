use std::path::PathBuf;

use anyhow::Result;

/// Cache pair trait.
pub trait CachePair {
    /// The magic number for the cache pair.
    const MAGIC_NUMBER: u64 = 0x1867C64E;
    /// The archive version for the cache pair.
    const ARCHIVE_VERSION: u64 = 20;

    /// Creates a new cache pair from the specified TOC and cache paths.
    fn new(toc_path: PathBuf, cache_path: PathBuf, is_post_ensmallening: bool) -> Self;

    /// Returns whether the package is post-ensmallening.
    ///
    /// This is used to determine how to decompress the data from before "The Great Ensmallening"
    /// update of Warframe.
    fn is_post_ensmallening(&self) -> bool;

    /// Returns the Table of Contents (TOC) file path.
    fn toc_path(&self) -> PathBuf;

    /// Returns the cache file path.
    fn cache_path(&self) -> PathBuf;

    /// Reads the TOC file.
    ///
    /// # Errors
    ///
    /// Returns an error if the TOC file cannot be read.
    fn read_toc(&mut self) -> Result<()>;

    /// Unreads the TOC file.
    ///
    /// This is used to reset the TOC file to the beginning.
    fn unread_toc(&mut self);
}
