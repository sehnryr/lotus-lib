/*!

This module implements the [`Node`] struct, which represents a file or directory in a structure that
mirrors the directory tree.

The [`Node`] struct implements two traits: [`FileNode`] and [`DirectoryNode`]. The [`FileNode`]
trait provides methods for getting information about a file, such as the cache offset, timestamp,
compressed length, and decompressed length. The [`DirectoryNode`] trait provides methods for getting
information about a directory, such as its children and a child with a given name.

*/

mod node;
mod toc;
mod toc_entry;

pub use node::{DirectoryNode, FileNode, Node, NodeKind};
pub(crate) use toc::Toc;
