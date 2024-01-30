use rctree::Node as RcNode;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Directory,
    File,
}

#[derive(Clone, Debug)]
pub struct Node {
    node: RcNode<NodeInner>,
}

impl Node {
    pub fn new(
        name: String,
        path: PathBuf,
        kind: NodeKind,
        cache_offset: Option<i64>,
        timestamp: Option<i64>,
        comp_len: Option<i32>,
        len: Option<i32>,
    ) -> Self {
        Self {
            node: RcNode::new(NodeInner::new(
                name,
                path,
                kind,
                cache_offset,
                timestamp,
                comp_len,
                len,
            )),
        }
    }

    pub fn root() -> Self {
        Self {
            node: RcNode::new(NodeInner::new(
                "".into(),
                PathBuf::from("/"),
                NodeKind::Directory,
                None,
                None,
                None,
                None,
            )),
        }
    }

    pub fn file(
        name: String,
        path: PathBuf,
        cache_offset: i64,
        timestamp: i64,
        comp_len: i32,
        len: i32,
    ) -> Self {
        Self::new(
            name,
            path,
            NodeKind::File,
            Some(cache_offset),
            Some(timestamp),
            Some(comp_len),
            Some(len),
        )
    }

    pub fn directory(name: String, path: PathBuf) -> Self {
        Self::new(name, path, NodeKind::Directory, None, None, None, None)
    }

    pub fn name(&self) -> String {
        self.node.borrow().name.clone()
    }

    pub fn path(&self) -> PathBuf {
        self.node.borrow().path.clone()
    }

    pub fn kind(&self) -> NodeKind {
        self.node.borrow().kind
    }

    pub(crate) fn append(&self, child: Node) {
        self.node.append(child.node)
    }

    pub fn parent(&self) -> Option<Node> {
        self.node.parent().map(|node| Self { node })
    }
}

pub trait FileNode {
    fn cache_offset(&self) -> i64;
    fn timestamp(&self) -> i64;
    fn comp_len(&self) -> i32;
    fn len(&self) -> i32;
}

pub trait DirectoryNode {
    fn children(&self) -> Vec<Node>;
    fn get_child(&self, name: &str) -> Option<Node>;
}

impl FileNode for Node {
    fn cache_offset(&self) -> i64 {
        self.node.borrow().cache_offset.unwrap()
    }

    fn timestamp(&self) -> i64 {
        self.node.borrow().timestamp.unwrap()
    }

    fn comp_len(&self) -> i32 {
        self.node.borrow().comp_len.unwrap()
    }

    fn len(&self) -> i32 {
        self.node.borrow().len.unwrap()
    }
}

impl DirectoryNode for Node {
    fn children(&self) -> Vec<Node> {
        self.node.children().map(|node| Self { node }).collect()
    }

    fn get_child(&self, name: &str) -> Option<Node> {
        self.children().into_iter().find(|node| node.name() == name)
    }
}

#[derive(Debug)]
struct NodeInner {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) kind: NodeKind,
    pub(crate) cache_offset: Option<i64>,
    pub(crate) timestamp: Option<i64>,
    pub(crate) comp_len: Option<i32>,
    pub(crate) len: Option<i32>,
}

impl NodeInner {
    pub fn new(
        name: String,
        path: PathBuf,
        kind: NodeKind,
        cache_offset: Option<i64>,
        timestamp: Option<i64>,
        comp_len: Option<i32>,
        len: Option<i32>,
    ) -> Self {
        Self {
            name,
            path,
            kind,
            cache_offset,
            timestamp,
            comp_len,
            len,
        }
    }
}
