use super::{dir_node::DirNode, node::Node};
use std::{rc::Rc, cell::RefCell};

pub struct FileNode {
    name: String,
    parent_node: Option<Rc<RefCell<DirNode>>>,
    cache_offset: i64,
    timestamp: i64,
    comp_len: i32,
    len: i32,
    toc_offset: i64,
}

impl Node for FileNode {
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
        let mut path = self.parent_node.clone().unwrap().borrow().path();
        path = format!("{}/{}", path, self.name);
        path
    }
}

impl FileNode {
    pub fn new(
        name: Option<String>,
        parent_node: Option<Rc<RefCell<DirNode>>>,
        cache_offset: Option<i64>,
        timestamp: Option<i64>,
        comp_len: Option<i32>,
        len: Option<i32>,
        toc_offset: Option<i64>,
    ) -> Self {
        Self {
            name: name.unwrap_or(String::new()),
            parent_node,
            cache_offset: cache_offset.unwrap_or(i64::MAX),
            timestamp: timestamp.unwrap_or(0),
            comp_len: comp_len.unwrap_or(0),
            len: len.unwrap_or(0),
            toc_offset: toc_offset.unwrap_or(0),
        }
    }

    pub fn set_data(
        &mut self,
        name: Option<String>,
        parent_node: Option<Rc<RefCell<DirNode>>>,
        cache_offset: Option<i64>,
        timestamp: Option<i64>,
        comp_len: Option<i32>,
        len: Option<i32>,
        toc_offset: Option<i64>,
    ) {
        if name.is_some() {
            self.name = name.unwrap();
        }
        if parent_node.is_some() {
            self.parent_node = parent_node;
        }
        if cache_offset.is_some() {
            self.cache_offset = cache_offset.unwrap();
        }
        if timestamp.is_some() {
            self.timestamp = timestamp.unwrap();
        }
        if comp_len.is_some() {
            self.comp_len = comp_len.unwrap();
        }
        if len.is_some() {
            self.len = len.unwrap();
        }
        if toc_offset.is_some() {
            self.toc_offset = toc_offset.unwrap();
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
