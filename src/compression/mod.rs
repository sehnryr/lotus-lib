mod lz;
mod oodle;
pub(crate) mod post_ensmallening;
pub(crate) mod pre_ensmallening;

#[cfg(feature = "post_ensmallening")]
pub use post_ensmallening::decompress_post_ensmallening;
#[cfg(feature = "pre_ensmallening")]
pub use pre_ensmallening::decompress_pre_ensmallening;

#[cfg(feature = "internal")]
pub use post_ensmallening::get_block_lengths;
#[cfg(feature = "internal")]
pub use post_ensmallening::is_oodle_block;
