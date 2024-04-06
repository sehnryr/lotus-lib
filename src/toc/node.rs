use std::fmt;
use std::path::PathBuf;

use arctree::Node as ArcNode;

/// The kind of a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    /// A directory node.
    Directory,

    /// A file node.
    File,
}

/// A node in a tree.
///
/// Nodes can be either directories or files.
/// The cost of cloning a node is low, as it uses [`Arc`] internally.
#[derive(Clone, PartialEq)]
pub struct Node {
    node: ArcNode<NodeData>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node.read();
        f.debug_struct("Node")
            .field("name", &node.name)
            .field("kind", &node.kind)
            .field("cache_offset", &node.cache_offset)
            .field("timestamp", &node.timestamp)
            .field("comp_len", &node.comp_len)
            .field("len", &node.len)
            .finish()
    }
}

/// Trait for file nodes.
pub trait FileNode {
    /// Returns the cache offset of the file.
    fn cache_offset(&self) -> i64;

    /// Returns the timestamp of the file.
    fn timestamp(&self) -> i64;

    /// Returns the compressed length of the file.
    fn comp_len(&self) -> i32;

    /// Returns the decompressed length of the file.
    fn len(&self) -> i32;
}

/// Trait for directory nodes.
pub trait DirectoryNode {
    /// Returns the children of the directory.
    fn children(&self) -> Vec<Node>;

    /// Returns the child with the given name.
    fn get_child(&self, name: &str) -> Option<Node>;
}

impl Node {
    fn new(
        name: &str,
        kind: NodeKind,
        cache_offset: Option<i64>,
        timestamp: Option<i64>,
        comp_len: Option<i32>,
        len: Option<i32>,
    ) -> Self {
        Self {
            node: ArcNode::new(NodeData::new(
                name,
                kind,
                cache_offset,
                timestamp,
                comp_len,
                len,
            )),
        }
    }

    pub(super) fn root() -> Self {
        Self::directory("")
    }

    pub(super) fn file(
        name: &str,
        cache_offset: i64,
        timestamp: i64,
        comp_len: i32,
        len: i32,
    ) -> Self {
        Self::new(
            name,
            NodeKind::File,
            Some(cache_offset),
            Some(timestamp),
            Some(comp_len),
            Some(len),
        )
    }

    pub(super) fn directory(name: &str) -> Self {
        Self::new(name, NodeKind::Directory, None, None, None, None)
    }

    pub(super) fn append(&mut self, child: Node) {
        self.node.append(child.node.clone());
    }

    /// Returns the name of the node.
    pub fn name(&self) -> String {
        self.node.read().name().to_string()
    }

    /// Returns the path of the node.
    pub fn path(&self) -> PathBuf {
        let mut ancestors = Vec::new();
        let mut ancestor = self.node.parent();
        while let Some(current_ancestor) = ancestor {
            ancestors.push(current_ancestor.clone());
            ancestor = current_ancestor.parent();
        }

        let mut path = PathBuf::from("/");
        for ancestor in ancestors.iter().rev() {
            path.push(ancestor.read().name());
        }

        path.push(self.name());

        path
    }

    /// Returns the kind of the node.
    pub fn kind(&self) -> NodeKind {
        self.node.read().kind()
    }

    /// Returns the parent of the node.
    pub fn parent(&self) -> Option<Node> {
        self.node.parent().map(|parent| Node { node: parent })
    }
}

impl FileNode for Node {
    fn cache_offset(&self) -> i64 {
        self.node.read().cache_offset().unwrap().clone()
    }

    fn timestamp(&self) -> i64 {
        self.node.read().timestamp().unwrap().clone()
    }

    fn comp_len(&self) -> i32 {
        self.node.read().comp_len().unwrap().clone()
    }

    fn len(&self) -> i32 {
        self.node.read().len().unwrap().clone()
    }
}

impl DirectoryNode for Node {
    fn children(&self) -> Vec<Node> {
        self.node
            .children()
            .map(|child| Node {
                node: child.clone(),
            })
            .collect()
    }

    fn get_child(&self, name: &str) -> Option<Node> {
        self.node
            .children()
            .find(|child| *child.read().name() == *name)
            .map(|child| Node {
                node: child.clone(),
            })
    }
}

#[derive(Debug)]
struct NodeData {
    name: String,
    kind: NodeKind,
    cache_offset: Option<i64>,
    timestamp: Option<i64>,
    comp_len: Option<i32>,
    len: Option<i32>,
}

impl NodeData {
    fn new(
        name: &str,
        kind: NodeKind,
        cache_offset: Option<i64>,
        timestamp: Option<i64>,
        comp_len: Option<i32>,
        len: Option<i32>,
    ) -> Self {
        Self {
            name: String::from(name),
            kind,
            cache_offset,
            timestamp,
            comp_len,
            len,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn kind(&self) -> NodeKind {
        self.kind
    }

    fn cache_offset(&self) -> Option<&i64> {
        self.cache_offset.as_ref()
    }

    fn timestamp(&self) -> Option<&i64> {
        self.timestamp.as_ref()
    }

    fn comp_len(&self) -> Option<&i32> {
        self.comp_len.as_ref()
    }

    fn len(&self) -> Option<&i32> {
        self.len.as_ref()
    }
}
