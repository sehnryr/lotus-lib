use anyhow::{Error, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Component, PathBuf};
use zerocopy::FromBytes;

use crate::toc::node::{DirectoryNode, Node, NodeKind};
use crate::toc::raw_toc_entry::{RawTocEntry, TOC_ENTRY_SIZE};

pub struct Toc {
    toc_path: PathBuf,
    directories: Vec<Node>,
    files: Vec<Node>,
    root: Option<Node>,
}

impl Toc {
    pub fn new(toc_path: PathBuf) -> Self {
        Self {
            toc_path,
            directories: Vec::new(),
            files: Vec::new(),
            root: None,
        }
    }

    pub fn directories(&self) -> Vec<Node> {
        self.directories.clone()
    }

    pub fn files(&self) -> Vec<Node> {
        self.files.clone()
    }

    pub fn root(&self) -> Option<Node> {
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

        let mut toc_reader = File::open(&self.toc_path).unwrap();
        let entry_count = (toc_reader.metadata().unwrap().len() as usize - 8) / TOC_ENTRY_SIZE;
        toc_reader.seek(SeekFrom::Start(8)).unwrap();

        // Reserve space for the entries in the vectors to avoid unnecessary
        // reallocations
        self.files.reserve(entry_count);
        self.directories.reserve(entry_count);

        let mut file_count = 0;
        let mut dir_count = 1; // Hardcoded root directory

        let root = Node::root();
        self.directories.insert(0, root.clone());

        let mut buffer = vec![0 as u8; TOC_ENTRY_SIZE * entry_count];
        toc_reader.read_exact(&mut buffer).unwrap();

        let entries = RawTocEntry::slice_from(&buffer).unwrap();
        for entry in entries {
            // Entry name is a null-terminated string, so we need to find the
            // index of the null byte and truncate the string there
            let entry_name = match entry.name.iter().position(|&x| x == 0) {
                Some(index) => String::from_utf8_lossy(&entry.name[0..index]),
                _ => String::from_utf8_lossy(&entry.name),
            }
            .into_owned();

            let parent_node = match self.directories.get(entry.parent_dir_index as usize) {
                Some(parent_node) => parent_node.clone(),
                _ => return Err(Error::msg("Failed to find parent directory")),
            };

            // Create the path for the entry
            let entry_path = parent_node.path().join(&entry_name);

            // If the cache offset is -1, then the entry is a directory
            if entry.cache_offset == -1 {
                let dir_node = Node::directory(entry_name, entry_path);

                self.directories.insert(dir_count, dir_node.clone());
                parent_node.append(dir_node);

                dir_count += 1;
            } else {
                let file_node = Node::file(
                    entry_name,
                    entry_path,
                    entry.cache_offset,
                    entry.timestamp,
                    entry.comp_len,
                    entry.len,
                );

                self.files.insert(file_count, file_node.clone());
                parent_node.append(file_node);

                file_count += 1;
            }
        }

        // Shrink the vectors to the actual size of the vectors to save memory
        self.directories.shrink_to_fit();
        self.files.shrink_to_fit();

        self.root = Some(root);

        Ok(()) // TOC read successfully
    }

    pub fn unread_toc(&mut self) {
        self.directories.clear();
        self.files.clear();
        self.root = None;
    }

    fn get_node(&self, path: PathBuf) -> Option<Node> {
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
                    current_node = match current_node.get_child(name) {
                        Some(child) => child,
                        _ => return None,
                    };
                }
                Component::ParentDir => {
                    current_node = match current_node.parent() {
                        Some(parent) => parent,
                        _ => return None,
                    };
                }
                Component::CurDir => continue,
                _ => return None,
            }
        }

        Some(current_node)
    }

    pub fn get_directory_node(&self, path: PathBuf) -> Option<Node> {
        match self.get_node(path) {
            Some(node) => match node.kind() {
                NodeKind::Directory => Some(node),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_file_node(&self, path: PathBuf) -> Option<Node> {
        match self.get_node(path) {
            Some(node) => match node.kind() {
                NodeKind::File => Some(node),
                _ => None,
            },
            _ => None,
        }
    }
}
