use std::future::Future;
use std::marker::PhantomData;
use std::path::Path;
use std::pin::Pin;
use std::task::{self, Context, Poll};
use std::{fmt, io};

use hyper::Uri;
use hyper_util::rt::TokioIo;
use pin_project_lite::pin_project;
use tokio::net::UnixStream;
use tower_service::Service;

use super::UnixSocketConnection;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// A target that [`UnixSocketConnector`] connects to.
pub trait UnixSocketTarget: sealed::Sealed {
    fn connect(&self) -> impl Future<Output = io::Result<UnixStream>> + Send + 'static;
}

impl<P: AsRef<Path> + ?Sized> sealed::Sealed for P {}

impl<P: AsRef<Path> + ?Sized> UnixSocketTarget for P {
    fn connect(&self) -> impl Future<Output = io::Result<UnixStream>> + Send + 'static {
        UnixStream::connect(self.as_ref().to_owned())
    }
}

/// A Connector for a Socket.
#[derive(Clone)]
pub struct UnixSocketConnector<T> {
    target: T,
}

impl<T: UnixSocketTarget> UnixSocketConnector<T> {
    /// Construct a new `UnixSocketConnector`.
    #[must_use]
    pub fn new(target: T) -> Self {
        Self { target }
    }
}

impl<T: UnixSocketTarget> From<T> for UnixSocketConnector<T> {
    fn from(target: T) -> UnixSocketConnector<T> {
        UnixSocketConnector { target }
    }
}

impl<T> fmt::Debug for UnixSocketConnector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnixSocketConnector")
            .finish_non_exhaustive()
    }
}

impl<T> Service<Uri> for UnixSocketConnector<T>
where
    T: UnixSocketTarget + Clone + Send,
{
    type Response = UnixSocketConnection;
    type Error = BoxError;
    type Future = UnixStreamConnecting;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Uri) -> Self::Future {
        let connecting = self.target.connect();

        let fut = async move {
            connecting
                .await
                .map(TokioIo::new)
                .map(Into::into)
                .map_err(Into::into)
        };

        UnixStreamConnecting {
            fut: Box::pin(fut),
            _marker: PhantomData,
        }
    }
}

type ConnectResult = Result<UnixSocketConnection, BoxError>;
type BoxConnecting = Pin<Box<dyn Future<Output = ConnectResult> + Send>>;

pin_project! {
    #[must_use = "futures do nothing unless polled"]
    pub struct UnixStreamConnecting<R = ()> {
        #[pin]
        fut: BoxConnecting,
        _marker: PhantomData<R>,
    }
}

impl<R> Future for UnixStreamConnecting<R> {
    type Output = ConnectResult;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        self.project().fut.poll(cx)
    }
}

impl<R> fmt::Debug for UnixStreamConnecting<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("UnixStreamConnecting")
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixListener;

    use hyper::Uri;
    use tower_service::Service as _;

    use super::UnixSocketConnector;

    #[tokio::test]
    async fn connects_to_pathname() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("hyper-unix-socket.sock");
        let _listener = UnixListener::bind(&path).unwrap();

        let mut connector = UnixSocketConnector::new(path);

        connector
            .call(Uri::from_static("http://localhost/"))
            .await
            .unwrap();
    }
}
