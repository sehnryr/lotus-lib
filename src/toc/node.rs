use super::DirectoryNode;
use std::{cell::RefCell, path::PathBuf, rc::Weak};

pub trait Node {
    fn name(&self) -> String;
    fn path(&self) -> PathBuf;
    fn parent_node(&self) -> Weak<RefCell<DirectoryNode>>;
}
