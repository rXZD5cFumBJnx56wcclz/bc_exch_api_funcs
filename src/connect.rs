use crate::prelude::*;

use futures::stream::{SplitSink, SplitStream};
use rustls_native_certs;
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_rustls::{
    TlsConnector,
    rustls::{ClientConfig, RootCertStore, pki_types::ServerName},
};
use tokio_tungstenite::client_async;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub type Wws = WebSocketStream<TlsStream<TcpStream>>;
pub type WwsSink = SplitSink<Wws, Message>;
pub type WwsStream = SplitStream<Wws>;

pub async fn connect_wws(
    addr: &str,
    url: &str,
) -> Result<(Wws, Response<Option<Vec<u8>>>), ExchangeError> {
    let tcp = TcpStream::connect(&format!("{addr}:443"))
        .await
        .map_err(|_| ExchangeError::NotConnected)?;
    tcp.set_nodelay(true)
        .map_err(|_e| ExchangeError::NotConnected)?;
    let mut roots = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
        roots.add(cert).map_err(|_e| ExchangeError::NotConnected)?;
    }
    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let server_name =
        ServerName::try_from(addr.to_string()).map_err(|_e| ExchangeError::NotConnected)?;
    let tls = connector
        .connect(server_name, tcp)
        .await
        .map_err(|_e| ExchangeError::NotConnected)?;
    Ok(client_async(url, tls)
        .await
        .map_err(|e| ExchangeError::WebSocket(e))?)
}

pub struct Connection(pub (WwsSink, WwsStream));

impl Connection {
    pub fn new(
        url: &str,
        s: &SETTINGS_EXCH,
        msg: Message,
    ) -> impl Future<Output = Result<(Self, Response<Option<Vec<u8>>>), ExchangeError>> {
        async move {
            let (mut ws, resp) = connect_wws(&s.wws_host, url)
                .await
                .map_err(|_e| ExchangeError::NotConnected)?;
            ws.send(msg)
                .await
                .map_err(|e| ExchangeError::WebSocket(e))?;
            Ok((Self(ws.split()), resp))
        }
    }
}

#[derive(Default, Clone)]
pub struct Connections(pub Vec<Arc<Mutex<Connection>>>);

impl Connections {
    pub fn new(
        url: &str,
        s: &SETTINGS_EXCH,
        msgs: Vec<Message>,
    ) -> impl Future<Output = Result<(Self, Vec<Response<Option<Vec<u8>>>>), ExchangeError>> {
        async move {
            let mut conn = Vec::with_capacity(msgs.len());
            let mut resp = Vec::with_capacity(msgs.len());
            for msg in msgs {
                let (connection, response) = Connection::new(url, s, msg).await?;
                conn.push(Arc::new(Mutex::new(connection)));
                resp.push(response);
            }
            Ok((Self(conn), resp))
        }
    }
    pub fn init(
        &mut self,
        url: &str,
        s: &SETTINGS_EXCH,
        msgs: Vec<Message>,
    ) -> impl Future<Output = Result<Vec<Response<Option<Vec<u8>>>>, ExchangeError>> {
        async move {
            let (conn, resp) = Connections::new(url, s, msgs).await?;
            *self = conn;
            Ok(resp)
        }
    }
}
