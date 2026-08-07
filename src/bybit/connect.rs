use crate::bybit::prelude::*;

fn msg(conn: &Connection, op: &str) -> impl Future<Output = Result<(), ExchangeError>> {
    async move {
        conn.0
            .lock()
            .await
            .0
            .send(Message::Text(
                format!(
                    r#"{{
                "op": "{op}"
                }}"#
                )
                .into(),
            ))
            .await
            .map_err(|e| ExchangeError::WebSocket(e))
    }
}

pub struct BybitConnection(pub Connection);

impl BybitConnection {
    pub fn new(
        url_path: &str,
        topic: &str,
        args: &[String],
        args_is_symbols: bool,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<(Self, Response<Option<Vec<u8>>>), ExchangeError>> {
        async move {
            let (conn, resp) = Connection::new(
                &format!("{}{url_path}", s.wws_url),
                s,
                get_subcribe_msg(topic, args, args_is_symbols, s),
            )
            .await?;
            Ok((Self(conn), resp))
        }
    }
    pub fn ping(&self) -> impl Future<Output = Result<(), ExchangeError>> {
        async move { msg(&self.0, "ping").await }
    }
    pub fn pong(&self) -> impl Future<Output = Result<(), ExchangeError>> {
        async move { msg(&self.0, "pong").await }
    }
}

#[derive(Default)]
pub struct BybitConnections(pub Connections);

impl BybitConnections {
    pub fn new(
        url_path: &str,
        topic: &str,
        args: Vec<&[String]>,
        args_is_symbols: bool,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<(Self, Vec<Response<Option<Vec<u8>>>>), ExchangeError>> {
        async move {
            let (conn, resp) = Connections::new(
                &format!("{}{url_path}", s.wws_url),
                s,
                get_subcribe_msgs(topic, args, args_is_symbols, s),
            )
            .await?;
            Ok((Self(conn), resp))
        }
    }
    pub fn ping(&self) -> impl Future<Output = Result<(), ExchangeError>> {
        async move {
            for conn in &self.0.0 {
                msg(conn, "ping").await?;
            }
            Ok(())
        }
    }
    pub fn pong(&self) -> impl Future<Output = Result<(), ExchangeError>> {
        async move {
            for conn in &self.0.0 {
                msg(conn, "pong").await?;
            }
            Ok(())
        }
    }
}
