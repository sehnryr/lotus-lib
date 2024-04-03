use std::path::PathBuf;
use std::sync::{Arc, RwLock, Weak};

use derivative::Derivative;

type Link<T> = Arc<RwLock<T>>;
type WeakLink<T> = Weak<RwLock<T>>;

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
#[derive(Clone, Debug)]
pub struct Node {
    node: Link<NodeData>,
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
            node: Arc::new(RwLock::new(NodeData::new(
                name,
                kind,
                cache_offset,
                timestamp,
                comp_len,
                len,
            ))),
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
        self.node.write().unwrap().append(child.node.clone());
        child.node.write().unwrap().set_parent(&self.node);
    }

    /// Returns the name of the node.
    pub fn name(&self) -> String {
        self.node.read().unwrap().name().to_string()
    }

    /// Returns the path of the node.
    pub fn path(&self) -> PathBuf {
        let mut path_components = Vec::new();
        let mut ancestor = self.node.read().unwrap().parent();

        while let Some(current_ancestor) = ancestor {
            let current_ancestor = current_ancestor.read().unwrap();
            path_components.push(current_ancestor.name());
            ancestor = current_ancestor.parent();
        }

        let mut path = PathBuf::from("/");
        for component in path_components.into_iter().rev() {
            path.push(component.to_string());
        }

        path.push(self.name());

        path
    }

    /// Returns the kind of the node.
    pub fn kind(&self) -> NodeKind {
        self.node.read().unwrap().kind()
    }

    /// Returns the parent of the node.
    pub fn parent(&self) -> Option<Node> {
        self.node
            .read()
            .unwrap()
            .parent()
            .map(|parent| Node { node: parent })
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.node, &other.node)
    }
}

impl FileNode for Node {
    fn cache_offset(&self) -> i64 {
        self.node.read().unwrap().cache_offset().unwrap().clone()
    }

    fn timestamp(&self) -> i64 {
        self.node.read().unwrap().timestamp().unwrap().clone()
    }

    fn comp_len(&self) -> i32 {
        self.node.read().unwrap().comp_len().unwrap().clone()
    }

    fn len(&self) -> i32 {
        self.node.read().unwrap().len().unwrap().clone()
    }
}

impl DirectoryNode for Node {
    fn children(&self) -> Vec<Node> {
        self.node
            .read()
            .unwrap()
            .children()
            .iter()
            .map(|child| Node {
                node: child.clone(),
            })
            .collect()
    }

    fn get_child(&self, name: &str) -> Option<Node> {
        self.node
            .read()
            .unwrap()
            .children()
            .iter()
            .find(|child| {
                let child = child.read().unwrap();
                *child.name() == *name
            })
            .map(|child| Node {
                node: child.clone(),
            })
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct NodeData {
    #[derivative(Debug = "ignore")]
    parent: Option<WeakLink<NodeData>>,
    #[derivative(Debug = "ignore")]
    children: Vec<Link<NodeData>>,

    name: Arc<str>,
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
            parent: None,
            children: Vec::new(),
            name: Arc::from(name),
            kind,
            cache_offset,
            timestamp,
            comp_len,
            len,
        }
    }

    fn set_parent(&mut self, parent: &Link<NodeData>) {
        self.parent = Some(Arc::downgrade(parent));
    }

    fn append(&mut self, child: Link<NodeData>) {
        self.children.push(child);
    }

    fn parent(&self) -> Option<Link<NodeData>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }

    fn children(&self) -> &Vec<Link<NodeData>> {
        &self.children
    }

    fn name(&self) -> Arc<str> {
        self.name.clone()
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
