use crate::exchs::bybit::prelude::*;

pub fn get_url_wws(is_public: bool, use_category: bool, s: &SETTINGS_EXCH) -> String {
    let path = if is_public { WS_PUBLIC } else { WS_PRIVATE };
    let category = if use_category {
        s.category.as_str()
    } else {
        ""
    };
    format!("{}{path}/{category}", s.wws_url.as_str())
}

pub fn get_subcribe_msg(
    topic: &str,
    args: &[String],
    args_is_symbols: bool,
    s: &SETTINGS_EXCH,
) -> Message {
    Message::Text(
        format!(
            r#"{{
                "op": "subscribe",
                "args": {}
            }}"#,
            if args_is_symbols {
                format!(
                    "{:?}",
                    args.iter()
                        .map(|v| format!("{topic}.{}.{v}", s.timeframe_sec.as_secs() / 60))
                        .collect::<Vec<String>>()
                )
            } else {
                format!("{:?}", args)
            }
        )
        .into(),
    )
}

pub fn get_subcribe_msgs(
    topic: &str,
    args: Vec<&[String]>,
    args_is_symbols: bool,
    s: &SETTINGS_EXCH,
) -> Vec<Message> {
    args.into_iter()
        .map(|v| get_subcribe_msg(topic, v, args_is_symbols, s))
        .collect()
}

fn msg(conn: &mut Connection, op: &str) -> impl Future<Output = Result<(), ExchangeError>> {
    async move {
        conn.0
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

pub fn new_connection(
    url_path: &str,
    topic: &str,
    args: &[String],
    args_is_symbols: bool,
    s: &SETTINGS_EXCH,
) -> impl Future<Output = Result<(Connection, Response<Option<Vec<u8>>>), ExchangeError>> {
    async move {
        let (conn, resp) = Connection::new(
            &format!("{}{url_path}", s.wws_url),
            s,
            get_subcribe_msg(topic, args, args_is_symbols, s),
        )
        .await?;
        Ok((conn, resp))
    }
}
pub fn ping(conn: &mut Connection) -> impl Future<Output = Result<(), ExchangeError>> {
    async move { msg(conn, "ping").await }
}
pub fn pong(conn: &mut Connection) -> impl Future<Output = Result<(), ExchangeError>> {
    async move { msg(conn, "pong").await }
}

pub fn new_connections(
    url_path: &str,
    topic: &str,
    args: Vec<&[String]>,
    args_is_symbols: bool,
    s: &SETTINGS_EXCH,
) -> impl Future<Output = Result<(Connections, Vec<Response<Option<Vec<u8>>>>), ExchangeError>> {
    async move {
        let (conn, resp) = Connections::new(
            &format!("{}{url_path}", s.wws_url),
            s,
            get_subcribe_msgs(topic, args, args_is_symbols, s),
        )
        .await?;
        Ok((conn, resp))
    }
}
