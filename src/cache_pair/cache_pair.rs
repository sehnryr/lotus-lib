pub trait CachePair {
    const MAGIC_NUMBER: u64 = 0x1867C64E;
    const ARCHIVE_VERSION: u64 = 20;

    fn new(
        toc_path: Option<String>,
        cache_path: Option<String>,
        is_post_ensmallening: bool,
    ) -> Self;
    fn is_post_ensmallening(&self) -> bool;
    fn toc_path(&self) -> String;
    fn cache_path(&self) -> String;
    fn read_toc(&self);
    fn unread_toc(&self);
}
