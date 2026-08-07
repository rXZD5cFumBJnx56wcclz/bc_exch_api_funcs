use crate::bybit::prelude::*;

pub fn get_url_wws(is_public: bool, use_category: bool, s: &SETTINGS_EXCH) -> String {
    let path = if is_public { WS_PUBLIC } else { WS_PRIVATE };
    let category = if use_category {
        s.category.as_str()
    } else {
        ""
    };
    format!("{}{path}/{category}", s.wws_url.as_str())
}

// msg method impl for every exch
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
