use super::package::Package;
use crate::cache_pair::CachePair;
use std::collections::HashMap;

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
        for entry in std::fs::read_dir(package_directory).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();

            let start = file_name.find('.');
            let end = file_name.rfind('.');

            if start.is_none() || end.is_none() || start.unwrap() == end.unwrap() {
                continue;
            }

            let mut directory = entry.path();
            directory.pop();

            let package_name = file_name[start.unwrap() + 1..end.unwrap()].to_string();
            let package: Package<T> =
                Package::new(directory, package_name.clone(), self.is_post_ensmallening);
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
