use std::{fmt::Debug, time::Duration};

#[derive(Debug)]
pub struct ResultWrap<T> {
    pub topic: Option<String>,
    pub time: Duration,
    pub res: T,
    pub info: Option<String>,
}
