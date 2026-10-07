#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use abstract_name::AbstractName;
pub use client::{UnixSocketConnector, UnixSocketTarget};
pub use stream::UnixSocketConnection;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod abstract_name;
mod client;
mod stream;
