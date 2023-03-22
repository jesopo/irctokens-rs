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

fn tag_decode(input: &str) -> String {
    let mut escaped = false;
    let mut output = String::new();

    for char in input.chars() {
        if escaped {
            escaped = false;
            let replace = match char {
                ':' => ';',
                's' => ' ',
                'r' => '\r',
                'n' => '\n',
                _ => char,
            };

            output.push(replace);
        } else if char == 0x5c as char {
            // backslash
            escaped = true;
        } else {
            output.push(char);
        }
    }

    output
}

pub fn tokenise(mut line: &[u8]) -> Result<Line, Error> {
    let tags = match line.first() {
        Some(b'@') => {
            let mut tags = &line.take_word(b' ')[1..];
            let mut tags_map = HashMap::new();

            while !tags.is_empty() {
                let mut tag_key_value = tags.take_word(b';');
                let tag_key = String::from_utf8(tag_key_value.take_word(b'=').to_vec())
                    .map_err(|_| Error::TagKeyDecode)?;
                let tag_value = match tag_key_value {
                    b"" | b"=" => None,
                    _ => Some(
                        std::str::from_utf8(tag_key_value)
                            .map(tag_decode)
                            .map_err(|_| Error::TagValueDecode)?,
                    ),
                };

                tags_map.insert(tag_key, tag_value);
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
