/// Represents the type of a package trio
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageType {
    /// Header package
    ///
    /// Usually contains file headers and metadata
    H,

    /// File package
    ///
    /// Usually contains compressed sound and texture assets
    F,

    /// Binary package
    ///
    /// Contains compiled binary data
    B,
}

impl TryFrom<String> for PackageType {
    type Error = &'static str;

    /// Converts from a [`String`].
    /// Calls [`Self::try_from(&str)`] on the string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not exactly one character long
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&str> for PackageType {
    type Error = &'static str;

    /// Converts from a [`str`] reference.
    /// Calls [`Self::try_from(char)`] on the first character of the string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not exactly one character long
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 1 {
            return Err("Invalid package trio type");
        }
        let c = s.chars().next().unwrap();
        Self::try_from(c)
    }
}

impl TryFrom<char> for PackageType {
    type Error = &'static str;

    /// Converts from a [`char`]
    ///
    /// # Errors
    ///
    /// Returns an error if the character is not 'H', 'F', or 'B'
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'H' | 'h' => Ok(Self::H),
            'F' | 'f' => Ok(Self::F),
            'B' | 'b' => Ok(Self::B),
            _ => Err("Invalid package trio type"),
        }
    }
}

impl From<PackageType> for char {
    /// Converts from a [`PackageType`] to a [`char`]
    fn from(package_type: PackageType) -> Self {
        match package_type {
            PackageType::H => 'H',
            PackageType::F => 'F',
            PackageType::B => 'B',
        }
    }
}
