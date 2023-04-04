use std::{cmp::Ordering, fmt};

/// Semantic Versioning Comparator
pub enum Comparator {
    /// Less Than
    Lt,
    /// Less Than or Equal To
    Lte,
    /// Greater Than
    Gt,
    /// Greater Than or Equal To
    Gte,
    /// Equal To
    Eq,
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comparator::Lt => write!(f, "<"),
            Comparator::Lte => write!(f, "<="),
            Comparator::Gt => write!(f, ">"),
            Comparator::Gte => write!(f, ">="),
            Comparator::Eq => write!(f, "="),
        }
    }
}

/// Semantic Versioning
pub struct Version {
    /// Major Version
    pub major: usize,
    /// Minor Version
    pub minor: usize,
    /// Patch Version
    pub patch: usize,
    /// Optional Comparator
    pub comparator: Option<Comparator>,
}

impl Version {
    const CURRENT: Self = Self { comparator: None, major: 0, minor: 4, patch: 0 };

    /// Creates a new version struct
    pub fn new(major: usize, minor: usize, patch: usize, comparator: Option<Comparator>) -> Self {
        Self { major, minor, patch, comparator }
    }

    /// Returns true if the version is valid
    ///
    /// If the comparator is None, then the version is valid if it is equal to the current version.
    pub fn is_valid(&self) -> bool {
        if self.comparator.is_none() {
            return self == &Self::CURRENT
        }

        match self.comparator.as_ref().unwrap() {
            Comparator::Lt => self < &Self::CURRENT,
            Comparator::Lte => self <= &Self::CURRENT,
            Comparator::Gt => self > &Self::CURRENT,
            Comparator::Gte => self >= &Self::CURRENT,
            Comparator::Eq => self == &Self::CURRENT,
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.major == other.major {
            if self.minor == other.minor {
                if self.patch == other.patch {
                    return Some(Ordering::Equal)
                }
                return Some(self.patch.cmp(&other.patch))
            }
            return Some(self.minor.cmp(&other.minor))
        }

        Some(self.major.cmp(&other.major))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let comparator = match &self.comparator {
            Some(c) => c.to_string(),
            None => "".to_string(),
        };
        write!(f, "{}{}.{}.{}", comparator, self.major, self.minor, self.patch)
    }
}
