use std::collections::BTreeMap;

/// A struct representing all the constituent pieces of an RFC1459/IRCv3 protocol line.
///
/// `@tagkey=tagvalue :source COMMAND arg1 arg2 :arg3 with space`
#[derive(Debug)]
pub struct Line {
    /// [Message tags] of an IRC line.
    /// [`None`] if no message tags were present.
    /// keys and values are [`String`] because they are promised to be utf8 encoded.
    ///
    /// [Message tags]: https://ircv3.net/specs/extensions/message-tags.html
    pub tags: Option<BTreeMap<String, Option<String>>>,
    /// The `:source` of an IRC line, or [`None`] if source is not present.
    /// This is a [`Vec<u8>`] as it may be unpredictably encoded.
    pub source: Option<Vec<u8>>,
    /// The `COMMAND` of an IRC line (e.g. `PRIVMSG`.)
    /// This is a [`String`] because commands are promised to be ascii encoded.
    pub command: String,
    /// The arguments of an IRC line.
    /// These are [`Vec<u8>`]s as they may be unpredictably encoded.
    pub arguments: Vec<Vec<u8>>,
}
