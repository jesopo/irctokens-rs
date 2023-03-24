use std::collections::{BTreeMap, VecDeque};

use super::util::TakeWord as _;
use super::Line;

const TAG_STOP: [&[u8]; 2] = [b"", b"="];

#[derive(Debug)]
pub enum Error {
    /// An empty byte array was passed to the tokeniser.
    Empty,
    /// A line is invalid if it has no `COMMAND` (e.g. `PRIVMSG`).
    MissingCommand,
    /// Commands must be ascii encoded.
    CommandDecode,
    /// Message tag keys must be utf8 encoded.
    TagKeyDecode,
    /// Message tag values must be utf8 encoded.
    TagValueDecode,
}

fn tag_decode(input: &str) -> String {
    let mut escaped = false;
    let mut output = String::with_capacity(input.len());

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
        } else if char == '\\' {
            // backslash
            escaped = true;
        } else {
            output.push(char);
        }
    }

    output
}

impl Line {
    #[allow(clippy::doc_markdown)]
    /// Attempt to tokenise a byte string by [RFC1459] and [IRCv3] protocol rules.
    ///
    /// Expects a byte string that does NOT contain a trailing CRLF.
    ///
    /// [RFC1459]: https://www.rfc-editor.org/rfc/rfc1459#section-2.3
    /// [IRCv3]: https://ircv3.net/specs/extensions/message-tags.html
    pub fn tokenise(line: impl std::convert::AsRef<[u8]>) -> Result<Self, Error> {
        let mut line = line.as_ref();
        let tags = if line.first() == Some(&b'@') {
            let mut tags = &line.take_word(b' ')[1..];
            let mut tags_map = BTreeMap::new();

            while !tags.is_empty() {
                let mut tag_key_value = tags.take_word(b';');
                let tag_key = String::from_utf8(tag_key_value.take_word(b'=').to_vec())
                    .map_err(|_| Error::TagKeyDecode)?;
                let tag_value = if TAG_STOP.contains(&tag_key_value) {
                    None
                } else {
                    Some(
                        std::str::from_utf8(tag_key_value)
                            .map(tag_decode)
                            .map_err(|_| Error::TagValueDecode)?,
                    )
                };

                tags_map.insert(tag_key, tag_value);
            }

            Some(tags_map)
        } else {
            None
        };

        let source = (line.first() == Some(&b':')).then(|| line.take_word(b' ')[1..].to_vec());

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

        Ok(Self {
            tags,
            source,
            command: String::from_utf8(command).map_err(|_| Error::CommandDecode)?,
            arguments: args.into(),
        })
    }
}

/// Implementation that simply calls [`Line::tokenise()`].
impl TryFrom<&[u8]> for Line {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::tokenise(value)
    }
}
