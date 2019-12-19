use std::fmt;

use crate::SStringCow;
use crate::SStringRef;

type StdString = std::string::String;

/// A UTF-8 encoded, immutable string.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct SString {
    #[serde(with = "serde_string")]
    pub(crate) inner: SStringInner,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SStringInner {
    Owned(StdString),
    Singleton(&'static str),
}

impl SString {
    /// Create a new empty `SString`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create an owned `SString`.
    pub fn owned(other: impl Into<StdString>) -> Self {
        Self {
            inner: SStringInner::Owned(other.into()),
        }
    }

    /// Create a reference to a `'static` data.
    pub fn singleton(other: &'static str) -> Self {
        Self {
            inner: SStringInner::Singleton(other),
        }
    }

    /// Get a reference to the `SString`.
    pub fn as_ref(&self) -> SStringRef<'_> {
        match self.inner {
            SStringInner::Owned(ref s) => SStringRef::borrow(s),
            SStringInner::Singleton(ref s) => SStringRef::singleton(s),
        }
    }

    /// Extracts a string slice containing the entire `SString`.
    pub fn as_str(&self) -> &str {
        match self.inner {
            SStringInner::Owned(ref s) => s.as_str(),
            SStringInner::Singleton(ref s) => s,
        }
    }

    /// Convert to a mutable string type, cloning the data if necessary.
    pub fn into_mut(self) -> StdString {
        match self.inner {
            SStringInner::Owned(s) => s,
            SStringInner::Singleton(s) => s.to_owned(),
        }
    }
}

impl std::ops::Deref for SString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl Eq for SString {}

impl<'s> PartialEq<SString> for SString {
    #[inline]
    fn eq(&self, other: &SString) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl<'s> PartialEq<str> for SString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl<'s> PartialEq<&'s str> for SString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl<'s> PartialEq<String> for SString {
    #[inline]
    fn eq(&self, other: &StdString) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl Ord for SString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for SString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl std::hash::Hash for SString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl fmt::Display for SString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<str> for SString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for SString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<std::ffi::OsStr> for SString {
    fn as_ref(&self) -> &std::ffi::OsStr {
        (&**self).as_ref()
    }
}

impl AsRef<std::path::Path> for SString {
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self)
    }
}

impl std::borrow::Borrow<str> for SString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Default for SString {
    fn default() -> Self {
        "".into()
    }
}

impl<'s> From<SStringRef<'s>> for SString {
    fn from(other: SStringRef<'s>) -> Self {
        other.to_owned()
    }
}

impl<'s> From<&'s SStringRef<'s>> for SString {
    fn from(other: &'s SStringRef<'s>) -> Self {
        other.to_owned()
    }
}

impl<'s> From<SStringCow<'s>> for SString {
    fn from(other: SStringCow<'s>) -> Self {
        other.into_owned()
    }
}

impl<'s> From<&'s SStringCow<'s>> for SString {
    fn from(other: &'s SStringCow<'s>) -> Self {
        other.clone().into_owned()
    }
}

impl From<StdString> for SString {
    fn from(other: StdString) -> Self {
        SString {
            inner: SStringInner::Owned(other),
        }
    }
}

impl From<&'static str> for SString {
    fn from(other: &'static str) -> Self {
        SString {
            inner: SStringInner::Singleton(other),
        }
    }
}

mod serde_string {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub(crate) fn serialize<S>(data: &SStringInner, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match data {
            SStringInner::Owned(ref s) => s.as_str(),
            SStringInner::Singleton(ref s) => s,
        };
        serializer.serialize_str(&s)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<SStringInner, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = StdString::deserialize(deserializer)?;
        Ok(SStringInner::Owned(s))
    }
}