use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Weak;

use crate::toc::DirectoryNode;

pub trait Node {
    fn name(&self) -> String;
    fn path(&self) -> PathBuf;
    fn parent_node(&self) -> Weak<RefCell<DirectoryNode>>;
}
