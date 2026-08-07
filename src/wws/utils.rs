use crate::prelude::*;

use rustls_native_certs;
use tokio::net::TcpStream;
use tokio_rustls::{
    TlsConnector,
    rustls::{ClientConfig, RootCertStore, pki_types::ServerName},
};
use tokio_tungstenite::client_async;

pub async fn connect_wws(
    addr: &str,
    url: &str,
) -> Result<(Wws, Response<Option<Vec<u8>>>), ExchangeError> {
    let tcp = TcpStream::connect(&format!("{addr}:443"))
        .await
        .map_err(|_| ExchangeError::NotConnected)?;
    tcp.set_nodelay(true)
        .map_err(|e| ExchangeError::NotConnected)?;
    let mut roots = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
        roots.add(cert).map_err(|e| ExchangeError::NotConnected)?;
    }
    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let server_name =
        ServerName::try_from(addr.to_string()).map_err(|e| ExchangeError::NotConnected)?;
    let tls = connector
        .connect(server_name, tcp)
        .await
        .map_err(|e| ExchangeError::NotConnected)?;
    Ok(client_async(url, tls)
        .await
        .map_err(|e| ExchangeError::WebSocket(e))?)
}
