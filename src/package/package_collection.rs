use std::collections::HashMap;

use crate::cache_pair::CachePair;
use crate::package::package::Package;

pub struct PackageCollection<T: CachePair> {
    directory: std::path::PathBuf,
    is_post_ensmallening: bool,
    packages: HashMap<String, Package<T>>,
}

impl<T: CachePair> PackageCollection<T> {
    pub fn new(directory: std::path::PathBuf, is_post_ensmallening: bool) -> Self {
        let mut package_collection = Self {
            directory,
            is_post_ensmallening,
            packages: HashMap::new(),
        };
        package_collection.load_packages();
        package_collection
    }

    fn load_packages(&mut self) {
        let package_directory = self.directory.clone();
        for entry in std::fs::read_dir(package_directory.clone()).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();

            // Check if the file has enough characters to be a package
            // 7 characters counts for the shortest possible package name of
            // 1 character : H.1.toc
            if file_name.len() < 7 {
                continue;
            }

            // Check if the file is a .toc file
            if !file_name.ends_with(".toc") {
                continue;
            }

            // Check if the file is a valid package
            if !(file_name.starts_with("H.")
                || file_name.starts_with("B.")
                || file_name.starts_with("F."))
            {
                continue;
            }

            let package_name = file_name[2..file_name.len() - 4].to_string();

            let package = Package::new(
                package_directory.clone(),
                package_name.clone(),
                self.is_post_ensmallening,
            );
            self.packages.insert(package_name, package);
        }
    }

    pub fn is_post_ensmallening(&self) -> bool {
        self.is_post_ensmallening
    }

    pub fn get_package(&self, package_name: &str) -> Option<&Package<T>> {
        self.packages.get(package_name)
    }

    pub fn directory(&self) -> &std::path::PathBuf {
        &self.directory
    }

    pub fn packages(&self) -> &HashMap<String, Package<T>> {
        &self.packages
    }
}
