use std::collections::{HashMap, VecDeque};

pub struct Line {
    // tags are promised to be utf8 encoded
    pub tags: Option<HashMap<String, Option<String>>>,
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

trait TakeWord<'a> {
    fn take_word(&mut self, sep: u8) -> &'a [u8];
}

impl<'a> TakeWord<'a> for &'a [u8] {
    fn take_word(&mut self, sep: u8) -> &'a [u8] {
        if let Some(i) = self.iter().position(|c| c == &sep) {
            let word = &self[..i];
            *self = &self[i + 1..];
            word
        } else {
            let word = &self[..];
            *self = &self[self.len()..];
            word
        }
    }
}

pub fn tokenise(mut line: &[u8]) -> Result<Line, Error> {
    let tags = match line.first() {
        Some(b'@') => {
            let mut tags = &line.take_word(b' ')[1..];
            let mut tags_map = HashMap::new();

            while !tags.is_empty() {
                let mut keyvalue = tags.take_word(b';');
                let tag = keyvalue.take_word(b'=');
                tags_map.insert(
                    String::from_utf8(tag.to_vec()).map_err(|_| Error::TagKeyDecode)?,
                    match keyvalue {
                        b"" | b"=" => None,
                        _ => Some(
                            String::from_utf8(keyvalue.to_vec())
                                .map_err(|_| Error::TagValueDecode)?,
                        ),
                    },
                );
            }

            Some(tags_map)
        }
        _ => None,
    };

    let source = match line.first() {
        Some(b':') => Some(line.take_word(b' ')[1..].to_vec()),
        _ => None,
    };

    let mut args = VecDeque::<Vec<u8>>::new();
    while !line.is_empty() {
        if line[0] == b':' {
            args.push_back(line[1..].to_vec());
            line = &[];
        } else {
            args.push_back(line.take_word(b' ').to_vec());
        }
    }

    let command = args.pop_front().ok_or(Error::MissingCommand)?;

    Ok(Line {
        tags,
        source,
        command: String::from_utf8(command).map_err(|_| Error::CommandDecode)?,
        args: args.into(),
    })
}
