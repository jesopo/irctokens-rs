use std::collections::BTreeMap;

pub struct Line {
    // tags are promised to be utf8 encoded
    pub tags: Option<BTreeMap<String, Option<String>>>,
    pub source: Option<Vec<u8>>,
    // commands are promised to be ascii encoded
    pub command: String,
    pub args: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum Error {
    Empty,
    MissingCommand,
    CommandDecode,
    TagKeyDecode,
    TagValueDecode,
}
