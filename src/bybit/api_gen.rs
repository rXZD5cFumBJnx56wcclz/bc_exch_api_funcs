use bc_utils_core::hashing::hmac_;

pub fn api_gen(
    key: &'static str,
    timestamp: &'static str,
    recv_window: &'static str,
    query_str: &'static str,
) -> String {
    hmac_(
        key.as_bytes(),
        format!("{timestamp}{key}{recv_window}{query_str}").as_bytes(),
    )
}
