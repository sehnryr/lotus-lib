use std::path::PathBuf;

use crate::cache_pair::CachePair;

use super::package_type::PackageType;

/// A package containing three cache pairs.
pub struct Package<T: CachePair> {
    directory: PathBuf,
    name: String,
    is_post_ensmallening: bool,

    h_package: Option<T>,
    f_package: Option<T>,
    b_package: Option<T>,
}

impl<T: CachePair> Package<T> {
    pub(super) fn new<P>(directory: P, name: String, is_post_ensmallening: bool) -> Self
    where
        P: Into<PathBuf>,
    {
        let directory = directory.into();

        let h_package = Package::<T>::new_package(&directory, &name, is_post_ensmallening, 'H');
        let f_package = Package::<T>::new_package(&directory, &name, is_post_ensmallening, 'F');
        let b_package = Package::<T>::new_package(&directory, &name, is_post_ensmallening, 'B');
        Self {
            directory,
            name,
            is_post_ensmallening,
            h_package,
            f_package,
            b_package,
        }
    }

    /// Creates a new package from the specified directory, name, and type.
    ///
    /// Returns `None` if the package does not exist.
    fn new_package<P, I>(
        directory: P,
        name: &str,
        is_post_ensmallening: bool,
        trio_type: I,
    ) -> Option<T>
    where
        P: Into<PathBuf>,
        I: TryInto<PackageType>,
    {
        let directory = directory.into();

        let mut toc_path = directory.clone();
        let mut cache_path = directory.clone();

        let trio_type = trio_type.try_into().ok()?;
        let trio_type = char::from(trio_type);

        toc_path.push(format!("{}.{}.toc", trio_type, name));
        cache_path.push(format!("{}.{}.cache", trio_type, name));

        if !toc_path.exists() && !cache_path.exists() {
            return None;
        }
        Some(T::new(toc_path, cache_path, is_post_ensmallening))
    }

    /// Returns the directory of the package.
    pub fn directory(&self) -> &PathBuf {
        &self.directory
    }

    /// Returns the name of the package.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns whether the package is post-ensmallening.
    ///
    /// This is used to determine how to decompress the data from before "The Great Ensmallening"
    /// update of Warframe.
    pub fn is_post_ensmallening(&self) -> bool {
        self.is_post_ensmallening
    }

    /// Returns a reference to the package of the specified type.
    ///
    /// Returns `None` if the package does not exist.
    pub fn borrow<I>(&self, package_type: I) -> Option<&T>
    where
        I: TryInto<PackageType>,
    {
        match package_type.try_into() {
            Ok(PackageType::H) => self.h_package.as_ref(),
            Ok(PackageType::F) => self.f_package.as_ref(),
            Ok(PackageType::B) => self.b_package.as_ref(),
            Err(_) => None,
        }
    }

    /// Returns a mutable reference to the package of the specified type.
    ///
    /// Returns `None` if the package does not exist.
    pub fn borrow_mut<I>(&mut self, package_type: I) -> Option<&mut T>
    where
        I: TryInto<PackageType>,
    {
        match package_type.try_into() {
            Ok(PackageType::H) => self.h_package.as_mut(),
            Ok(PackageType::F) => self.f_package.as_mut(),
            Ok(PackageType::B) => self.b_package.as_mut(),
            Err(_) => None,
        }
    }

    /// Returns the package of the specified type.
    pub fn take<I>(&mut self, package_type: I) -> Option<T>
    where
        I: TryInto<PackageType>,
    {
        match package_type.try_into() {
            Ok(PackageType::H) => self.h_package.take(),
            Ok(PackageType::F) => self.f_package.take(),
            Ok(PackageType::B) => self.b_package.take(),
            Err(_) => None,
        }
    }
}
