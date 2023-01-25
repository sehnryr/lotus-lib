use super::{file_node::FileNode, node::Node};
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

pub struct DirNode {
    name: String,
    parent_node: Option<Rc<RefCell<DirNode>>>,
    child_dirs: HashMap<String, Rc<RefCell<DirNode>>>,
    child_files: HashMap<String, Rc<RefCell<FileNode>>>,
    toc_offset: i64,
}

impl Node for DirNode {
    fn name(&self) -> &String {
        &self.name
    }

    fn parent(&self) -> Option<Rc<RefCell<dyn Node>>> {
        match &self.parent_node {
            Some(parent) => Some(parent.to_owned()),
            None => None,
        }
    }

    fn toc_offset(&self) -> i64 {
        self.toc_offset
    }

    fn path(&self) -> String {
        let mut path = self.name.clone();
        if self.parent_node.is_some() {
            path = format!(
                "{}/{}",
                self.parent_node.clone().unwrap().borrow().path(),
                path
            );
        }
        path
    }
}

impl DirNode {
    pub fn new(
        name: Option<String>,
        parent_node: Option<Rc<RefCell<DirNode>>>,
        toc_offset: Option<i64>,
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
        parent_node: Option<Rc<RefCell<DirNode>>>,
        toc_offset: Option<i64>,
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

    pub fn get_child_dir(&self, name: &str) -> Option<Rc<RefCell<DirNode>>> {
        match self.child_dirs.get(name) {
            Some(dir) => Some(dir.to_owned()),
            None => None,
        }
    }

    pub fn get_child_file(&self, name: &str) -> Option<Rc<RefCell<FileNode>>> {
        match self.child_files.get(name) {
            Some(file) => Some(file.to_owned()),
            None => None,
        }
    }

    pub fn child_dirs(&self) -> &HashMap<String, Rc<RefCell<DirNode>>> {
        &self.child_dirs
    }

    pub fn child_files(&self) -> &HashMap<String, Rc<RefCell<FileNode>>> {
        &self.child_files
    }

    pub fn add_child_dir(&mut self, dir: Rc<RefCell<DirNode>>) {
        self.child_dirs.insert(dir.clone().as_ref().borrow().name().to_string(), dir);
    }

    pub fn add_child_file(&mut self, file: Rc<RefCell<FileNode>>) {
        self.child_files
            .insert(file.clone().as_ref().borrow().name().to_string(), file);
    }
}
