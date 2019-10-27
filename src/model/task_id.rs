use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct TaskID(String);

impl TaskID {
    pub fn new<I: AsRef<str>>(id: I) -> Self { Self(id.as_ref().to_string()) }
}

impl From<String> for TaskID {
    fn from(string: String) -> Self { Self(string) }
}

impl Into<String> for TaskID {
    fn into(self) -> String { self.0 }
}
