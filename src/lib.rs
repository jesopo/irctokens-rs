//! ## usage
//!
//! ### tokenisation
//! 
//! ```
//! let bytes = b"@id=123 :jess!~jess@hostname PRIVMSG #chat :hello there!";
//! let line = irctokens::Line::tokenise(bytes).unwrap();
//! println!("{:?}", line.tags);
//! println!("{:?}", line.source);
//! println!("{}", line.command);
//! println!("{:?}", line.arguments);
//! ```

mod format;
mod obj;
pub mod tokenise;
mod util;

pub use self::obj::Line;
