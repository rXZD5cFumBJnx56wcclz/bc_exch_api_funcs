use crate::prelude::*;

use std::sync::Arc;

use rustls_native_certs;
use tokio::net::TcpStream;
use tokio_rustls::{
    TlsConnector,
    client::TlsStream,
    rustls::{ClientConfig, RootCertStore, pki_types::ServerName},
};
use tokio_tungstenite::{WebSocketStream, client_async};

pub async fn connect_wws(
    addr: &str,
    url: &str,
    port: &str,
) -> Result<
    (
        WebSocketStream<TlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    ),
    Box<dyn Error>,
> {
    let tcp = TcpStream::connect(&format!("{addr}:{port}")).await?;
    tcp.set_nodelay(true)?;
    let mut roots = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
        roots.add(cert)?;
    }
    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let server_name = ServerName::try_from("stream.bybit.com")?;
    let tls = connector.connect(server_name, tcp).await?;
    Ok(client_async(url, tls).await?)
}
