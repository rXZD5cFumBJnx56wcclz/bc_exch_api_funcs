use crate::prelude::*;

use futures::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub type Wws = WebSocketStream<TlsStream<TcpStream>>;
pub type WwsSink = SplitSink<Wws, Message>;
pub type WwsStream = SplitStream<Wws>;

pub struct Connection(pub Mutex<(WwsSink, WwsStream)>);

impl Connection {
    pub fn new(
        url: &str,
        s: &SETTINGS_EXCH,
        msg: Message,
    ) -> impl Future<Output = Result<(Self, Response<Option<Vec<u8>>>), ExchangeError>> {
        async move {
            let (mut ws, resp) = connect_wws(&s.wws_host, url)
                .await
                .map_err(|e| ExchangeError::NotConnected)?;
            ws.send(msg)
                .await
                .map_err(|e| ExchangeError::WebSocket(e))?;
            Ok((Self(Mutex::new(ws.split())), resp))
        }
    }
}

#[derive(Default, Clone)]
pub struct Connections(pub Vec<Arc<Connection>>);

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
                conn.push(Arc::new(connection));
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
