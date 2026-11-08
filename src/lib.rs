//! A Unix socket connector for [`hyper`].
//!
//! [`UnixSocketConnector`] connects a [`hyper_util::client::legacy::Client`] to one of:
//!
//! - a socket path, any `AsRef<Path>`
#![cfg_attr(
    any(target_os = "linux", target_os = "android"),
    doc = "- a name in the abstract socket namespace, see [`AbstractName`] (Linux and Android only)"
)]
//!
//! # Example
//!
//! ```no_run
//! use bytes::Bytes;
//! use http_body_util::Empty;
//! use hyper_unix_socket::UnixSocketConnector;
//! use hyper_util::client::legacy::Client;
//! use hyper_util::rt::TokioExecutor;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let connector = UnixSocketConnector::new("/var/run/docker.sock");
//!
//!     let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(connector);
//!
//!     // every request goes to the socket, whatever the host in the URI
//!     let response = client
//!         .get("http://localhost/containers/json".parse()?)
//!         .await?;
//!
//!     println!("Status: {}", response.status());
//!
//!     Ok(())
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use abstract_name::AbstractName;
pub use client::{UnixSocketConnector, UnixSocketTarget};
pub use stream::UnixSocketConnection;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod abstract_name;
mod client;
mod stream;
