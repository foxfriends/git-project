use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct TaskID(String);

impl TaskID {
    pub fn new<I: AsRef<str>>(id: I) -> Self { Self(id.as_ref().to_string()) }
}

