use super::{node::Node, DirectoryNode};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::{Rc, Weak},
};

#[derive(Clone, Debug)]
pub struct FileNode {
    name: String,
    path: PathBuf,
    parent_node: Weak<RefCell<DirectoryNode>>,
    cache_offset: i64,
    timestamp: i64,
    comp_len: i32,
    len: i32,
}

impl FileNode {
    pub fn new(
        name: String,
        parent_node: Rc<RefCell<DirectoryNode>>,
        cache_offset: i64,
        timestamp: i64,
        comp_len: i32,
        len: i32,
    ) -> Self {
        let mut path = parent_node.borrow().path().clone();
        path.push(name.clone());
        Self {
            name,
            path,
            parent_node: Rc::downgrade(&parent_node),
            cache_offset,
            timestamp,
            comp_len,
            len,
        }
    }

    pub fn cache_offset(&self) -> i64 {
        self.cache_offset
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn comp_len(&self) -> i32 {
        self.comp_len
    }

    pub fn len(&self) -> i32 {
        self.len
    }
}

impl Node for FileNode {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn parent_node(&self) -> Weak<RefCell<DirectoryNode>> {
        self.parent_node.clone()
    }
}
