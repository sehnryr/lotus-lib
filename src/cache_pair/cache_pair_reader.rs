use super::cache_pair::CachePair;
use crate::toc::{dir_node::DirNode, directory_tree::DirectoryTree, file_node::FileNode};
use std::{cell::RefCell, rc::Rc};

pub struct CachePairReader {
    is_post_ensmallening: bool,
    toc_path: std::path::PathBuf,
    cache_path: std::path::PathBuf,
    directory_tree: Rc<RefCell<DirectoryTree>>,
}

impl CachePair for CachePairReader {
    fn new(
        toc_path: Option<String>,
        cache_path: Option<String>,
        is_post_ensmallening: bool,
    ) -> Self {
        let toc_path = std::path::PathBuf::from(toc_path.unwrap());
        let cache_path = std::path::PathBuf::from(cache_path.unwrap());
        let directory_tree = Rc::new(RefCell::new(DirectoryTree::new(toc_path.clone())));
        Self {
            is_post_ensmallening,
            toc_path,
            cache_path,
            directory_tree,
        }
    }

    fn is_post_ensmallening(&self) -> bool {
        self.is_post_ensmallening
    }

    fn toc_path(&self) -> String {
        self.toc_path.to_str().unwrap().to_string()
    }

    fn cache_path(&self) -> String {
        self.cache_path.to_str().unwrap().to_string()
    }

    fn read_toc(&self) {
        self.directory_tree.borrow_mut().read_toc();
    }

    fn unread_toc(&self) {
        self.directory_tree.borrow_mut().unread_toc();
    }
}

impl CachePairReader {
    pub fn get_dir_node(&self, path: &str) -> Option<Rc<RefCell<DirNode>>> {
        self.directory_tree.borrow().get_dir_node(path)
    }

    pub fn get_file_node(&self, path: &str) -> Option<Rc<RefCell<FileNode>>> {
        self.directory_tree.borrow().get_file_node(path)
    }

    pub fn print_tree(&self) {
        self.directory_tree.borrow().print_tree(None);
    }
}
