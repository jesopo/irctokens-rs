use super::Line;

fn tag_encode(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);

    for char in input.chars() {
        output.push_str(&match char {
            ';' => "\\:".to_string(),
            ' ' => "\\s".to_string(),
            '\\' => "\\".to_string(),
            '\r' => "\\r".to_string(),
            '\n' => "\\n".to_string(),
            _ => char.to_string(),
        });
    }

    output
}

impl Line {
    #[allow(clippy::doc_markdown)]
    /// Format `self` in to a byte string by [RFC1459] and [IRCv3] protocol rules.
    ///
    /// [RFC1459]: https://www.rfc-editor.org/rfc/rfc1459#section-2.3
    /// [IRCv3]: https://ircv3.net/specs/extensions/message-tags.html
    #[must_use]
    pub fn format(&self) -> Vec<u8> {
        let mut output = Vec::new();

        if let Some(tags) = &self.tags {
            output.push(b'@');
            for (i, (key, value)) in tags.iter().enumerate() {
                if i != 0 {
                    output.push(b';');
                }

                output.extend_from_slice(key.as_bytes());
                if let Some(value) = value {
                    output.push(b'=');
                    output.extend_from_slice(tag_encode(value).as_bytes());
                }
            }
            output.push(b' ');
        }

        if let Some(source) = &self.source {
            output.push(b':');
            output.extend_from_slice(source);
            output.push(b' ');
        }

        output.extend_from_slice(self.command.as_bytes());

        for (i, arg) in self.arguments.iter().enumerate() {
            output.push(b' ');
            if i == self.arguments.len() - 1 {
                output.push(b':');
            }
            output.extend_from_slice(arg);
        }

        output
    }
}
