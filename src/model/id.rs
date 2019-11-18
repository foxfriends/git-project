use std::fmt::{self, Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Id(String);

impl Id {
    pub fn new<I: AsRef<str>>(id: I) -> Self { Self(id.as_ref().to_string()) }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Id {
    fn from(string: &str) -> Self { Self(string.to_string()) }
}

impl From<String> for Id {
    fn from(string: String) -> Self { Self(string) }
}

impl Into<String> for Id {
    fn into(self) -> String { self.0 }
}

impl Into<String> for &Id {
    fn into(self) -> String { self.0.to_string() }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str { &self.0 }
}
