use super::{dir_node::DirNode, file_node::FileNode, node::Node, raw_toc_entry::RawTocEntry};
use std::{
    cell::RefCell,
    io::{Read, Seek},
    mem,
    rc::Rc,
};

pub struct DirectoryTree {
    toc_path: std::path::PathBuf,
    dirs: Vec<Rc<RefCell<DirNode>>>,
    files: Vec<Rc<RefCell<FileNode>>>,
    root_node: Option<Rc<RefCell<DirNode>>>,
    has_read: bool,
}

impl DirectoryTree {
    pub fn new(toc_path: std::path::PathBuf) -> Self {
        Self {
            toc_path: toc_path,
            dirs: Vec::new(),
            files: Vec::new(),
            root_node: None,
            has_read: false,
        }
    }

    pub fn read_toc(&mut self) {
        if self.has_read {
            return;
        }

        const TOC_ENTRY_LEN: usize = mem::size_of::<RawTocEntry>();

        let mut toc_reader = std::fs::File::open(&self.toc_path).unwrap();
        let entry_count = (toc_reader.metadata().unwrap().len() as usize - 8) / TOC_ENTRY_LEN;
        toc_reader.seek(std::io::SeekFrom::Start(8)).unwrap();

        self.files.reserve(entry_count);
        self.dirs.reserve(entry_count);

        let mut file_count = 0;
        let mut dir_count = 1;

        self.dirs
            .insert(0, Rc::new(RefCell::new(DirNode::new(None, None, None))));

        for i in 0..entry_count {
            let mut buffer = [0 as u8; TOC_ENTRY_LEN];
            toc_reader.read_exact(&mut buffer).unwrap();
            let entry = unsafe { mem::transmute::<[u8; TOC_ENTRY_LEN], RawTocEntry>(buffer) };

            let entry_name = std::str::from_utf8(&entry.name)
                .unwrap()
                .trim_end_matches('\0')
                .to_string();
            let parent_dir = self.dirs.get(entry.parent_dir_index as usize);
            let parent_dir = match parent_dir.is_some() {
                true => Some(Rc::clone(parent_dir.unwrap())),
                false => None,
            };

            if entry.cache_offset == -1 {
                let dir_node = Rc::new(RefCell::new(DirNode::new(
                    Some(entry_name),
                    parent_dir.clone(),
                    Some(i as i64),
                )));
                self.dirs.insert(dir_count, Rc::clone(&dir_node));

                if parent_dir.is_some() {
                    let parent_dir = parent_dir.unwrap();
                    parent_dir.borrow_mut().add_child_dir(Rc::clone(&dir_node));
                }

                dir_count += 1;
            } else {
                let file_node = Rc::new(RefCell::new(FileNode::new(
                    Some(entry_name),
                    parent_dir.clone(),
                    Some(entry.cache_offset),
                    Some(entry.timestamp),
                    Some(entry.comp_len),
                    Some(entry.len),
                    Some(i as i64),
                )));
                self.files.insert(file_count, Rc::clone(&file_node));

                if parent_dir.is_some() {
                    let parent_dir = parent_dir.unwrap();
                    parent_dir
                        .borrow_mut()
                        .add_child_file(Rc::clone(&file_node));
                }

                file_count += 1;
            }
        }

        self.dirs.shrink_to_fit();
        self.files.shrink_to_fit();

        self.root_node = Some(Rc::clone(&self.dirs[0]));

        self.has_read = true;
    }

    pub fn unread_toc(&mut self) {
        self.dirs.clear();
        self.files.clear();
        self.root_node = None;
        self.has_read = false;
    }

    pub fn get_file_node(&self, path: &str) -> Option<Rc<RefCell<FileNode>>> {
        let mut path = path.split('/').collect::<Vec<&str>>();
        let file_name = path.pop().unwrap().to_string();

        let dir_node = self.get_dir_node(path.join("/").as_str());
        if dir_node.is_none() {
            return None;
        }

        let dir_node = dir_node.unwrap();
        let binding = dir_node.borrow();
        let child_files = binding.child_files();
        let file_entry = child_files.get(&file_name);

        if file_entry.is_none() {
            return None;
        }

        return Some(Rc::clone(file_entry.unwrap()));
    }

    pub fn get_dir_node(&self, path: &str) -> Option<Rc<RefCell<DirNode>>> {
        let mut path = path.split('/').collect::<Vec<&str>>();
        path.remove(0);

        let mut current_dir = self.root_node.clone();

        for dir_name in path.clone() {
            if current_dir.clone().is_none() {
                return None;
            }

            let _current_dir = current_dir.clone().unwrap();
            let _current_dir = _current_dir.borrow();
            let child_dir = _current_dir.child_dirs().get(dir_name);

            if child_dir.is_none() {
                return None;
            }

            current_dir = Some(Rc::clone(child_dir.unwrap()));
        }
        return current_dir;
    }

    pub fn files(&self) -> &Vec<Rc<RefCell<FileNode>>> {
        &self.files
    }

    pub fn dirs(&self) -> &Vec<Rc<RefCell<DirNode>>> {
        &self.dirs
    }

    pub fn print_tree(&self, mut root_node: Option<Rc<RefCell<DirNode>>>) {
        if root_node.is_none() {
            root_node = self.root_node.clone();
        }

        let root_node = root_node.unwrap();

        for (_, child_dir) in root_node.borrow().child_dirs() {
            println!("{}", child_dir.borrow().path());
            self.print_tree(Some(Rc::clone(child_dir)));
        }

        for (_, child_file) in root_node.borrow().child_files() {
            println!("{}", child_file.borrow().path());
        }
    }
}
