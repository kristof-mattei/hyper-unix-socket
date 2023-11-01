use std::fmt;
use std::io;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};

use hyper::rt::{Read, ReadBufCursor, Write};

use hyper_util::{
    client::connect::{Connected, Connection},
    rt::TokioIo,
};
use tokio::net::UnixStream;

/// A stream that goes over a Unix socket.
pub struct UnixSocketConnection(TokioIo<UnixStream>);

// ===== impl UnixSocketConnection =====

impl fmt::Debug for UnixSocketConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("UnixSocketConnection")
            .field(&self.0)
            .finish()
    }
}

impl From<TokioIo<UnixStream>> for UnixSocketConnection {
    fn from(inner: TokioIo<UnixStream>) -> Self {
        Self(inner)
    }
}

impl Read for UnixSocketConnection {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: ReadBufCursor<'_>,
    ) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut Pin::get_mut(self).0).poll_read(cx, buf)
    }
}

impl Write for UnixSocketConnection {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut Pin::get_mut(self).0).poll_write(cx, buf)
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut Pin::get_mut(self).0).poll_write_vectored(cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut Pin::get_mut(self).0).poll_flush(cx)
    }

    #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut Pin::get_mut(self).0).poll_shutdown(cx)
    }
}

impl Connection for UnixSocketConnection {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}
