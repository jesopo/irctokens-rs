use std::io::Write;

use super::Line;

fn tag_encode(input: &str, output: &mut (impl Write + ?Sized)) -> std::io::Result<()> {
    for char in input.chars() {
        match char {
            ';' => write!(output, "\\:")?,
            ' ' => write!(output, "\\s")?,
            '\\' => write!(output, "\\")?,
            '\r' => write!(output, "\\r")?,
            '\n' => write!(output, "\\n")?,
            _ => write!(output, "{char}")?,
        }
    }
    Ok(())
}

impl Line {
    #[allow(clippy::doc_markdown)]
    /// Write `self` to `output` as a formatted byte string by [RFC1459] and [IRCV3] protocol rules.
    ///
    /// Does NOT write a CRLF nor flush the stream.
    /// This function makes a large number of small writes;
    /// it is advised to use a buffered [`Write`] implementation here.
    ///
    /// [RFC1459]: https://www.rfc-editor.org/rfc/rfc1459#section-2.3
    /// [IRCv3]: https://ircv3.net/specs/extensions/message-tags.html
    pub fn write_to(&self, output: &mut (impl Write + ?Sized)) -> std::io::Result<()> {
        if let Some(tags) = &self.tags {
            let mut not_at_start = false;
            for (key, value) in tags {
                if not_at_start {
                    write!(output, ";{key}")?;
                } else {
                    not_at_start = true;
                    write!(output, "@{key}")?;
                }
                if let Some(value) = value {
                    output.write_all(b"=")?;
                    tag_encode(value, output)?;
                }
            }
            output.write_all(b" ")?;
        }

        if let Some(source) = &self.source {
            output.write_all(b":")?;
            output.write_all(source)?;
            output.write_all(b" ")?;
        }

        output.write_all(self.command.as_bytes())?;

        if let Some((last, args)) = self.arguments.split_last() {
            for arg in args {
                output.write_all(b" ")?;
                output.write_all(arg)?;
            }
            output.write_all(b" :")?;
            output.write_all(last)?;
        }
        Ok(())
    }
    #[allow(clippy::doc_markdown)]
    /// Format `self` into a byte string by [RFC1459] and [IRCv3] protocol rules.
    ///
    /// The returned byte string is NOT suffixed with a CRLF.
    ///
    /// [RFC1459]: https://www.rfc-editor.org/rfc/rfc1459#section-2.3
    /// [IRCv3]: https://ircv3.net/specs/extensions/message-tags.html
    #[must_use]
    pub fn format(&self) -> Vec<u8> {
        // Minimum size of a message is its command's length plus 2 bytes per argument.
        // In practice reallocation is basically guaranteed, but this provides a starting point.
        let mut output = Vec::with_capacity(self.command.len() + self.arguments.len() * 2);
        std::mem::drop(self.write_to(&mut output));
        output
    }
}
