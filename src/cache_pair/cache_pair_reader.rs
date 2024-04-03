use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use anyhow::Result;

use crate::cache_pair::cache_pair::CachePair;
use crate::compression::post_ensmallening::decompress_post_ensmallening;
use crate::compression::pre_ensmallening::decompress_pre_ensmallening;
use crate::toc::{FileNode, Node, Toc};

pub struct CachePairReader {
    is_post_ensmallening: bool,
    toc_path: PathBuf,
    cache_path: PathBuf,
    toc: Toc,
}

impl CachePair for CachePairReader {
    fn new(toc_path: PathBuf, cache_path: PathBuf, is_post_ensmallening: bool) -> Self {
        let toc = Toc::new(toc_path.clone());
        Self {
            is_post_ensmallening,
            toc_path,
            cache_path,
            toc,
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

    fn read_toc(&mut self) -> Result<()> {
        self.toc.read_toc()
    }

    fn unread_toc(&mut self) {
        self.toc.unread_toc()
    }
}

impl CachePairReader {
    pub fn get_directory_node<T: Into<PathBuf>>(&self, path: T) -> Option<Node> {
        self.toc.get_directory_node(path.into())
    }

    pub fn get_file_node<T: Into<PathBuf>>(&self, path: T) -> Option<Node> {
        self.toc.get_file_node(path.into())
    }

    pub fn directories(&self) -> &Vec<Node> {
        self.toc.directories()
    }

    pub fn files(&self) -> &Vec<Node> {
        self.toc.files()
    }

    pub fn get_data(&self, file_node: Node) -> Result<Vec<u8>> {
        let mut cache_reader = File::open(self.cache_path.clone()).unwrap();
        cache_reader
            .seek(SeekFrom::Start(file_node.cache_offset() as u64))
            .unwrap();

        let mut data = vec![0; file_node.comp_len() as usize];
        cache_reader.read_exact(&mut data).unwrap();
        Ok(data)
    }

    pub fn decompress_data(&self, file_node: Node) -> Result<Vec<u8>> {
        if file_node.comp_len() == file_node.len() {
            return self.get_data(file_node);
        }

        let mut cache_reader = File::open(self.cache_path.clone()).unwrap();
        cache_reader.seek(SeekFrom::Start(file_node.cache_offset() as u64))?;

        if self.is_post_ensmallening {
            return decompress_post_ensmallening(
                file_node.comp_len() as usize,
                file_node.len() as usize,
                &mut cache_reader,
            );
        } else {
            return decompress_pre_ensmallening(
                file_node.comp_len() as usize,
                file_node.len() as usize,
                &mut cache_reader,
            );
        }
    }
}
