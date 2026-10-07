use std::future::Future;
use std::io;
#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt as _;
#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt as _;
use std::os::unix::net::SocketAddr as StdSocketAddr;

use tokio::net::UnixStream;
use tokio::net::unix::SocketAddr;

use super::UnixSocketTarget;
use super::client::sealed::Sealed;

/// A name in the abstract socket namespace.
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
#[derive(Clone)]
pub struct AbstractName(SocketAddr);

impl AbstractName {
    /// Construct a new `AbstractName`.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is longer than `SUN_LEN - 1`.
    pub fn new<N: AsRef<[u8]>>(name: N) -> io::Result<Self> {
        StdSocketAddr::from_abstract_name(name)
            .map(SocketAddr::from)
            .map(Self)
    }
}

impl Sealed for AbstractName {}

impl UnixSocketTarget for AbstractName {
    fn connect(&self) -> impl Future<Output = io::Result<UnixStream>> + Send + 'static {
        let socket_addr = self.0.clone();

        async move { UnixStream::connect_addr(&socket_addr).await }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "android")]
    use std::os::android::net::SocketAddrExt as _;
    #[cfg(target_os = "linux")]
    use std::os::linux::net::SocketAddrExt as _;
    use std::os::unix::net::{SocketAddr, UnixListener};

    use hyper::Uri;
    use tower_service::Service as _;

    use super::AbstractName;
    use crate::UnixSocketConnector;

    #[tokio::test]
    async fn connects_to_abstract_name() {
        let name = format!("hyper-unix-socket-{}", std::process::id());
        let socket_addr = SocketAddr::from_abstract_name(name.as_bytes()).unwrap();
        let _listener = UnixListener::bind_addr(&socket_addr).unwrap();

        let mut connector = UnixSocketConnector::new(AbstractName::new(name.as_bytes()).unwrap());

        connector
            .call(Uri::from_static("http://localhost/"))
            .await
            .unwrap();
    }

    #[test]
    fn rejects_oversized_name() {
        // `sun_path` is 108 bytes, including the leading NUL
        let name = [b'a'; 108];

        AbstractName::new(name).map(drop).unwrap_err();
    }
}
