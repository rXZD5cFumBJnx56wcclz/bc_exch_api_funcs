use bc_utils_lg::settings::SETTINGS;

#[derive(Debug, Clone)]
pub struct BYBIT<'a> {
    pub s: &'a SETTINGS,
}

impl<'a> BYBIT<'a> {
    pub fn new(s: &'a SETTINGS) -> Self {
        Self { s }
    }
}

pub trait Exchange<'a> {
    fn s(&'a self) -> &'a SETTINGS;
}

impl<'a> Exchange<'a> for BYBIT<'a> {
    fn s(&'a self) -> &'a SETTINGS {
        &self.s
    }
}
