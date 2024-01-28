use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use crate::toc::node::Node;
use crate::toc::FileNode;

#[derive(Clone, Debug)]
pub struct DirectoryNode {
    name: String,
    path: PathBuf,
    parent_node: Weak<RefCell<DirectoryNode>>,
    children_directories: Vec<Rc<RefCell<DirectoryNode>>>,
    children_files: Vec<Rc<RefCell<FileNode>>>,
}

impl DirectoryNode {
    pub fn new(name: String, parent_node: Rc<RefCell<DirectoryNode>>) -> Self {
        let mut path = parent_node.borrow().path.clone();
        path.push(name.clone());
        Self {
            name,
            path,
            parent_node: Rc::downgrade(&parent_node),
            children_directories: Vec::new(),
            children_files: Vec::new(),
        }
    }

    pub fn root() -> Self {
        Self {
            name: "".into(),
            path: PathBuf::from("/"),
            parent_node: Weak::new(),
            children_directories: Vec::new(),
            children_files: Vec::new(),
        }
    }

    pub fn children_directories(&self) -> Vec<Rc<RefCell<DirectoryNode>>> {
        self.children_directories.clone()
    }

    pub fn children_files(&self) -> Vec<Rc<RefCell<FileNode>>> {
        self.children_files.clone()
    }

    pub fn get_child_directory<T: Into<String>>(
        &self,
        name: T,
    ) -> Option<Rc<RefCell<DirectoryNode>>> {
        let name = name.into();
        self.children_directories
            .iter()
            .find(|directory_node| directory_node.borrow().name() == name)
            .map(|directory_node| directory_node.clone())
    }

    pub fn get_child_file<T: Into<String>>(&self, name: T) -> Option<Rc<RefCell<FileNode>>> {
        let name = name.into();
        self.children_files
            .iter()
            .find(|file_node| file_node.borrow().name() == name)
            .map(|file_node| file_node.clone())
    }
}

pub(crate) trait DirectoryNodeMut {
    fn add_child_directory(&mut self, directory_node: Rc<RefCell<DirectoryNode>>);
    fn add_child_file(&mut self, file_node: Rc<RefCell<FileNode>>);
}

impl DirectoryNodeMut for DirectoryNode {
    fn add_child_directory(&mut self, directory_node: Rc<RefCell<DirectoryNode>>) {
        self.children_directories.push(directory_node);
    }

    fn add_child_file(&mut self, file_node: Rc<RefCell<FileNode>>) {
        self.children_files.push(file_node);
    }
}

impl Node for DirectoryNode {
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
