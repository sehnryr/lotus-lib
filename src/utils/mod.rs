mod compression;
mod compression_lotus;

pub use compression_lotus::decompress_post_ensmallening;
pub use compression_lotus::decompress_pre_ensmallening;
pub use compression_lotus::internal_decompress_post_ensmallening;
pub use compression_lotus::internal_decompress_pre_ensmallening;

pub use compression_lotus::get_block_lengths;
pub use compression_lotus::is_oodle_block;
