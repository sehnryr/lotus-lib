use super::{node::Node, FileNode};
use indexmap::IndexMap;
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::{Rc, Weak},
};

#[derive(Clone, Debug)]
pub struct DirectoryNode {
    name: String,
    path: PathBuf,
    parent_node: Weak<RefCell<DirectoryNode>>,
    children_directories: IndexMap<String, Rc<RefCell<DirectoryNode>>>,
    children_files: IndexMap<String, Rc<RefCell<FileNode>>>,
}

impl DirectoryNode {
    pub fn new(name: String, parent_node: Rc<RefCell<DirectoryNode>>) -> Self {
        let mut path = parent_node.borrow().path.clone();
        path.push(name.clone());
        Self {
            name,
            path,
            parent_node: Rc::downgrade(&parent_node),
            children_directories: IndexMap::new(),
            children_files: IndexMap::new(),
        }
    }

    pub fn root() -> Self {
        Self {
            name: "".into(),
            path: PathBuf::from("/"),
            parent_node: Weak::new(),
            children_directories: IndexMap::new(),
            children_files: IndexMap::new(),
        }
    }

    pub fn children_directories(&self) -> Vec<Rc<RefCell<DirectoryNode>>> {
        self.children_directories.values().cloned().collect()
    }

    pub fn children_files(&self) -> Vec<Rc<RefCell<FileNode>>> {
        self.children_files.values().cloned().collect()
    }

    pub fn get_child_directory<T: Into<String>>(
        &self,
        name: T,
    ) -> Option<Rc<RefCell<DirectoryNode>>> {
        self.children_directories.get(&name.into()).cloned()
    }

    pub fn get_child_file<T: Into<String>>(&self, name: T) -> Option<Rc<RefCell<FileNode>>> {
        self.children_files.get(&name.into()).cloned()
    }
}

pub(crate) trait DirectoryNodeMut {
    fn add_child_directory(&mut self, directory_node: Rc<RefCell<DirectoryNode>>);
    fn add_child_file(&mut self, file_node: Rc<RefCell<FileNode>>);
}

impl DirectoryNodeMut for DirectoryNode {
    fn add_child_directory(&mut self, directory_node: Rc<RefCell<DirectoryNode>>) {
        self.children_directories
            .insert(directory_node.borrow().name(), directory_node.clone());
    }

    fn add_child_file(&mut self, file_node: Rc<RefCell<FileNode>>) {
        self.children_files
            .insert(file_node.borrow().name(), file_node.clone());
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
