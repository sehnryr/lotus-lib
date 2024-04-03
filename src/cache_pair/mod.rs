/*!

This module provides functionality to read and parse the cache pairs from the `Cache.Windows`
directory. Note that a cache pair is a pair of `.toc` and `.cache` files that contain information
about the contents of a package and the compressed data for the package, respectively.

*/

mod cache_pair;
mod cache_pair_reader;

pub use cache_pair::CachePair;
pub use cache_pair_reader::CachePairReader;
