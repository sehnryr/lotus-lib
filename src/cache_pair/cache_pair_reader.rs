use super::cache_pair::CachePair;
use crate::{
    toc::{DirNode, DirectoryTree, FileNode},
    utils::{decompress_post_ensmallening, decompress_pre_ensmallening},
};
use std::{
    cell::RefCell,
    io::{Read, Seek},
    rc::Rc,
};

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

    pub fn get_data(&self, entry: Rc<RefCell<FileNode>>) -> Vec<u8> {
        let file_node = entry.borrow();
        let mut cache_reader = std::fs::File::open(self.cache_path.clone()).unwrap();
        cache_reader
            .seek(std::io::SeekFrom::Start(file_node.cache_offset() as u64))
            .unwrap();

        let mut data = vec![0; file_node.comp_len() as usize];
        cache_reader.read_exact(&mut data).unwrap();
        data
    }

    pub fn decompress_data(&self, entry: Rc<RefCell<FileNode>>) -> Vec<u8> {
        let file_node = entry.borrow();
        if file_node.comp_len() == file_node.len() {
            return self.get_data(entry.clone());
        }

        let mut cache_reader = std::fs::File::open(self.cache_path.clone()).unwrap();

        if self.is_post_ensmallening {
            return decompress_post_ensmallening(entry.clone(), &mut cache_reader);
        } else {
            return decompress_pre_ensmallening(entry.clone(), &mut cache_reader);
        }
    }
}
