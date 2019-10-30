use std::fmt::{self, Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct TaskID(String);

impl TaskID {
    pub fn new<I: AsRef<str>>(id: I) -> Self { Self(id.as_ref().to_string()) }
}

impl Display for TaskID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TaskID {
    fn from(string: String) -> Self { Self(string) }
}

impl Into<String> for TaskID {
    fn into(self) -> String { self.0 }
}

impl Into<String> for &TaskID {
    fn into(self) -> String { self.0.to_string() }
}
