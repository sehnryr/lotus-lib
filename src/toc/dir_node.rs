use std::{collections::HashMap, rc::Rc};

use super::{file_node::FileNode, node::Node};

pub struct DirNode {
    name: String,
    parent_node: Option<Rc<DirNode>>,
    child_dirs: HashMap<String, Rc<DirNode>>,
    child_files: HashMap<String, Rc<FileNode>>,
    toc_offset: u64,
}

impl Node for DirNode {
    fn name(&self) -> &String {
        &self.name
    }

    fn parent(&self) -> Option<Rc<dyn Node>> {
        match &self.parent_node {
            Some(parent) => Some(parent.to_owned()),
            None => None,
        }
    }

    fn toc_offset(&self) -> u64 {
        self.toc_offset
    }

    fn path(&self) -> String {
        let mut path = String::new();
        let mut current_dir = self;
        while current_dir.parent_node.is_some() {
            path = format!("{}/{}", current_dir.name, path);
            current_dir = current_dir.parent_node.as_ref().unwrap();
        }
        path.insert(0, '/');
        path
    }
}

impl DirNode {
    pub fn new(
        &self,
        name: Option<String>,
        parent_node: Option<Rc<DirNode>>,
        toc_offset: Option<u64>,
    ) -> Self {
        Self {
            name: name.unwrap_or(String::new()),
            parent_node,
            child_dirs: HashMap::new(),
            child_files: HashMap::new(),
            toc_offset: toc_offset.unwrap_or(0),
        }
    }

    pub fn set_data(
        &mut self,
        name: Option<String>,
        parent_node: Option<Rc<DirNode>>,
        toc_offset: Option<u64>,
    ) {
        if name.is_some() {
            self.name = name.unwrap();
        }
        if parent_node.is_some() {
            self.parent_node = parent_node;
        }
        if toc_offset.is_some() {
            self.toc_offset = toc_offset.unwrap();
        }
    }

    pub fn dir_count(&self) -> usize {
        self.child_dirs.len()
    }

    pub fn file_count(&self) -> usize {
        self.child_files.len()
    }

    pub fn get_child_dir(&self, name: &str) -> Option<Rc<DirNode>> {
        match self.child_dirs.get(name) {
            Some(dir) => Some(dir.to_owned()),
            None => None,
        }
    }

    pub fn get_child_file(&self, name: &str) -> Option<Rc<FileNode>> {
        match self.child_files.get(name) {
            Some(file) => Some(file.to_owned()),
            None => None,
        }
    }

    pub fn add_child_dir(&mut self, dir: Rc<DirNode>) {
        self.child_dirs.insert(dir.name().to_owned(), dir);
    }

    pub fn add_child_file(&mut self, file: Rc<FileNode>) {
        self.child_files.insert(file.name().to_owned(), file);
    }
}
