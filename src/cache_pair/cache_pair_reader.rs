use super::cache_pair::CachePair;
use crate::toc::{DirectoryNode, DirectoryTree, FileNode};
use crate::utils::{decompress_post_ensmallening, decompress_pre_ensmallening};
use std::{
    cell::RefCell,
    io::{self, Read, Seek},
    path::PathBuf,
    rc::Rc,
};

pub struct CachePairReader {
    is_post_ensmallening: bool,
    toc_path: std::path::PathBuf,
    cache_path: std::path::PathBuf,
    directory_tree: Rc<RefCell<DirectoryTree>>,
}

impl CachePair for CachePairReader {
    fn new(toc_path: PathBuf, cache_path: PathBuf, is_post_ensmallening: bool) -> Self {
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

    fn toc_path(&self) -> PathBuf {
        self.toc_path.clone()
    }

    fn cache_path(&self) -> PathBuf {
        self.cache_path.clone()
    }

    fn read_toc(&self) -> Result<(), io::Error> {
        self.directory_tree.borrow_mut().read_toc()
    }

    fn unread_toc(&self) {
        self.directory_tree.borrow_mut().unread_toc();
    }
}

impl CachePairReader {
    pub fn get_directory_node<T: Into<PathBuf>>(
        &self,
        path: T,
    ) -> Option<Rc<RefCell<DirectoryNode>>> {
        self.directory_tree.borrow().get_directory_node(path.into())
    }

    pub fn get_file_node<T: Into<PathBuf>>(&self, path: T) -> Option<Rc<RefCell<FileNode>>> {
        self.directory_tree.borrow().get_file_node(path.into())
    }

    pub fn directories(&self) -> Vec<Rc<RefCell<DirectoryNode>>> {
        self.directory_tree.borrow().directories().to_vec()
    }

    pub fn files(&self) -> Vec<Rc<RefCell<FileNode>>> {
        self.directory_tree.borrow().files().to_vec()
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
