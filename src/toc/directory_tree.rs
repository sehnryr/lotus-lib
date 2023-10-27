use anyhow::{Error, Result};
use std::cell::RefCell;
use std::io::{Read, Seek};
use std::path::{Component, PathBuf};
use std::rc::Rc;

use crate::toc::directory_node::DirectoryNodeMut;
use crate::toc::node::Node;
use crate::toc::raw_toc_entry::{RawTocEntry, TOC_ENTRY_SIZE};
use crate::toc::{DirectoryNode, FileNode};

pub struct DirectoryTree {
    toc_path: std::path::PathBuf,
    directories: Vec<Rc<RefCell<DirectoryNode>>>,
    files: Vec<Rc<RefCell<FileNode>>>,
    root: Option<Rc<RefCell<DirectoryNode>>>,
}

impl DirectoryTree {
    pub fn new(toc_path: std::path::PathBuf) -> Self {
        Self {
            toc_path,
            directories: Vec::new(),
            files: Vec::new(),
            root: None,
        }
    }

    pub fn directories(&self) -> Vec<Rc<RefCell<DirectoryNode>>> {
        self.directories.clone()
    }

    pub fn files(&self) -> Vec<Rc<RefCell<FileNode>>> {
        self.files.clone()
    }

    pub fn root(&self) -> Option<Rc<RefCell<DirectoryNode>>> {
        self.root.clone()
    }

    pub fn is_loaded(&self) -> bool {
        self.root.is_some()
    }

    pub fn read_toc(&mut self) -> Result<()> {
        if self.is_loaded() {
            return Ok(()); // TOC already loaded
        }

        // Clear the directory and file vectors in case they were populated
        // from a previous read
        self.unread_toc();

        let mut toc_reader = std::fs::File::open(&self.toc_path).unwrap();
        let entry_count = (toc_reader.metadata().unwrap().len() as usize - 8) / TOC_ENTRY_SIZE;
        toc_reader.seek(std::io::SeekFrom::Start(8)).unwrap();

        // Reserve space for the entries in the vectors to avoid unnecessary
        // reallocations
        self.files.reserve(entry_count);
        self.directories.reserve(entry_count);

        let mut file_count = 0;
        let mut dir_count = 1;

        let root = Rc::new(RefCell::new(DirectoryNode::root()));
        self.directories.insert(0, root.clone());

        for _ in 0..entry_count {
            let mut buffer = [0 as u8; TOC_ENTRY_SIZE];
            toc_reader.read_exact(&mut buffer).unwrap();
            let entry = RawTocEntry::from(&buffer);

            // Entry name is a null-terminated string, so we need to find the
            // index of the null byte and truncate the string there
            let entry_name = match entry.name.iter().position(|&x| x == 0) {
                Some(index) => String::from_utf8_lossy(&entry.name[0..index]),
                _ => String::from_utf8_lossy(&entry.name),
            }
            .to_string();

            let parent_node = match self.directories.get(entry.parent_dir_index as usize) {
                Some(parent_node) => parent_node.clone(),
                _ => return Err(Error::msg("Failed to find parent directory")),
            };

            // If the cache offset is -1, then the entry is a directory
            if entry.cache_offset == -1 {
                let dir_node = Rc::new(RefCell::new(DirectoryNode::new(
                    entry_name,
                    parent_node.clone(),
                )));

                self.directories.insert(dir_count, dir_node.clone());
                parent_node
                    .borrow_mut()
                    .add_child_directory(dir_node.clone());

                dir_count += 1;
            } else {
                let file_node = Rc::new(RefCell::new(FileNode::new(
                    entry_name,
                    parent_node.clone(),
                    entry.cache_offset,
                    entry.timestamp,
                    entry.comp_len,
                    entry.len,
                )));

                self.files.insert(file_count, file_node.clone());
                parent_node.borrow_mut().add_child_file(file_node.clone());

                file_count += 1;
            }
        }

        // Shrink the vectors to the actual size of the vectors to save memory
        self.directories.shrink_to_fit();
        self.files.shrink_to_fit();

        self.root = Some(root.clone());

        Ok(()) // TOC read successfully
    }

    pub fn unread_toc(&mut self) {
        self.directories.clear();
        self.files.clear();
        self.root = None;
    }

    pub fn get_directory_node(&self, path: PathBuf) -> Option<Rc<RefCell<DirectoryNode>>> {
        if !self.is_loaded() {
            return None;
        }

        if !path.has_root() {
            panic!("Path must be absolute");
        }

        let mut components = path.components();
        let mut current_node = self.root.clone().unwrap();

        // Skip root
        components.next();

        for component in components {
            match component {
                Component::Normal(name) => {
                    let name = name.to_str().unwrap();
                    let child = current_node.borrow().get_child_directory(name);

                    // If child is not a directory, return None
                    if child.is_none() {
                        return None;
                    }

                    current_node = child.unwrap();
                }
                Component::ParentDir => {
                    let parent_node = current_node.borrow().parent_node();
                    current_node = match parent_node.upgrade() {
                        Some(parent_node) => parent_node,
                        _ => return None,
                    }
                }
                Component::CurDir => continue,
                _ => return None,
            }
        }

        Some(current_node)
    }

    pub fn get_file_node(&self, path: PathBuf) -> Option<Rc<RefCell<FileNode>>> {
        if !self.is_loaded() {
            return None;
        }

        if !path.has_root() {
            panic!("Path must be absolute");
        }

        let mut path = path.clone();

        let binding = path.clone();
        let file_name = binding.file_name();

        if file_name.is_none() {
            return None;
        }

        let file_name = file_name.unwrap().to_str().unwrap();
        path.pop();

        let dir_node = self.get_directory_node(path.clone());

        if dir_node.is_none() {
            return None;
        }

        let dir_node = dir_node.unwrap();
        let file_node = dir_node.borrow().get_child_file(file_name);

        file_node
    }
}
