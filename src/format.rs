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

        for (i, arg) in self.args.iter().enumerate() {
            output.push(b' ');
            if i == self.args.len() - 1 {
                output.push(b':');
            }
            output.extend_from_slice(arg);
        }

        output
    }
}
