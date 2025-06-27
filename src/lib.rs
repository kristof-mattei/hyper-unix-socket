#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(warnings)]
// exceptions
#![deny(let_underscore_drop)]
#![deny(non_ascii_idents)]
#![expect(clippy::uninlined_format_args)]

pub use client::UnixSocketConnector;
pub use stream::UnixSocketConnection;

mod client;
mod stream;
