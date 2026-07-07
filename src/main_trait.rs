use std::fmt::Debug;

use bc_utils_lg::structs::settings::SETTINGS;

pub trait Exchange {
    fn s<'a>(&'a self) -> &'a SETTINGS;
}

pub trait ResultWrap<T>: Debug {
    fn res(self) -> T;
}
