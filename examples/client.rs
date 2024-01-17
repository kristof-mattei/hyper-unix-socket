use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper_unix_socket::UnixSocketConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use tokio::io::{self, AsyncWriteExt as _};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unix_socket_connector = UnixSocketConnector::new("/var/run/docker.sock");

    let client =
        Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(unix_socket_connector);

    let mut res = client
        .get("http://localhost/containers/json".parse()?)
        .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    while let Some(frame) = res.body_mut().frame().await {
        let frame = frame?;

        if let Some(d) = frame.data_ref() {
            io::stdout().write_all(d).await?;
        }
    }

    Ok(())
}
