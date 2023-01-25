use super::cache_pair::CachePair;

pub enum PackageTrioType {
    H,
    F,
    B,
}

pub struct Package<T: CachePair> {
    directory: std::path::PathBuf,
    name: String,
    is_post_ensmallening: bool,
    packages: (Option<T>, Option<T>, Option<T>),
}

impl<T: CachePair> Package<T> {
    pub fn new(
        directory: std::path::PathBuf,
        name: String,
        is_post_ensmallening: bool,
    ) -> Self {
        let mut package = Self {
            directory,
            name,
            is_post_ensmallening,
            packages: (None, None, None),
        };
        package.load_package_pairs();
        package
    }

    fn load_package_pairs(&mut self) {
        for package_type in [PackageTrioType::H, PackageTrioType::F, PackageTrioType::B] {
            let (toc_path, cache_path) = self.get_pair_path(&package_type);
            if !toc_path.exists() && !cache_path.exists() {
                continue;
            }

            let package_pair = T::new(
                Some(toc_path.to_str().unwrap().to_string()),
                Some(cache_path.to_str().unwrap().to_string()),
                self.is_post_ensmallening,
            );

            match package_type {
                PackageTrioType::H => self.packages.0 = Some(package_pair),
                PackageTrioType::F => self.packages.1 = Some(package_pair),
                PackageTrioType::B => self.packages.2 = Some(package_pair),
            }
        }
    }

    fn get_pair_path(
        &self,
        package_type: &PackageTrioType,
    ) -> (std::path::PathBuf, std::path::PathBuf) {
        let mut toc_path = self.directory.clone();
        let mut cache_path = self.directory.clone();

        let trio_char = match package_type {
            PackageTrioType::H => "H",
            PackageTrioType::F => "F",
            PackageTrioType::B => "B",
        };

        toc_path.push(format!("{}.{}.toc", trio_char, self.name));
        cache_path.push(format!("{}.{}.cache", trio_char, self.name));

        (toc_path, cache_path)
    }

    pub fn directory(&self) -> &std::path::PathBuf {
        &self.directory
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_post_ensmallening(&self) -> bool {
        self.is_post_ensmallening
    }
}
