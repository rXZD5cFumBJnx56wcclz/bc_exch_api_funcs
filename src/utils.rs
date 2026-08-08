use crate::prelude::*;

pub fn symbols_splitted(symbols: &[String], num: usize) -> Vec<&[String]> {
    symbols.chunks(symbols.len().div_ceil(num)).collect()
}

pub fn new_rest_client(s: &SETTINGS_EXCH) -> Client {
    Client::builder().timeout(s.timeout_req_ms).build().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn symbols_splitted_res_1() {
        let bind = &[
            "ETHUSDT".to_string(),
            "ETHUSDT".to_string(),
            "ETHUSDT".to_string(),
            "ETHUSDT".to_string(),
            "ETHUSDT".to_string(),
        ];
        let symbols = symbols_splitted(bind, 2);
        assert_eq_pr!(symbols.len(), 2);
    }
}
