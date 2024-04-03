use std::io;
use std::path::PathBuf;

use crate::cache_pair::CachePair;
use crate::package::package::Package;

pub struct PackageCollection<T: CachePair> {
    directory: PathBuf,
    is_post_ensmallening: bool,
    packages: Vec<Package<T>>,
}

impl<T: CachePair> PackageCollection<T> {
    /// Creates a new package collection from the specified directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory does not exist or if the directory cannot be read.
    pub fn new<P>(directory: P, is_post_ensmallening: bool) -> Result<Self, io::Error>
    where
        P: Into<PathBuf>,
    {
        let directory = directory.into();

        let mut packages = Vec::new();
        for entry in std::fs::read_dir(&directory).unwrap() {
            let entry = entry?;
            let file_name = entry.file_name().into_string().unwrap();

            // Check if the file has enough characters to be a package 7 characters counts for the
            // shortest possible package name of 1 character : H.1.toc
            if file_name.len() < 7 {
                continue;
            }

            // Check if the file is a valid header .toc file
            if !file_name.starts_with("H.") || !file_name.ends_with(".toc") {
                continue;
            }

            let package_name = file_name[2..file_name.len() - 4].to_string();

            let package = Package::<T>::new(&directory, package_name, is_post_ensmallening);
            packages.push(package);
        }

        Ok(Self {
            directory,
            is_post_ensmallening,
            packages,
        })
    }

    pub fn is_post_ensmallening(&self) -> bool {
        self.is_post_ensmallening
    }

    /// Returns a reference to the package with the specified name if found.
    pub fn borrow(&self, package_name: &str) -> Option<&Package<T>> {
        self.packages
            .iter()
            .find(|package| package.name() == package_name)
    }

    /// Returns a mutable reference to the package with the specified name if found.
    pub fn borrow_mut(&mut self, package_name: &str) -> Option<&mut Package<T>> {
        self.packages
            .iter_mut()
            .find(|package| package.name() == package_name)
    }

    pub fn directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn packages(&self) -> &Vec<Package<T>> {
        &self.packages
    }
}
