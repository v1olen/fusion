use serde::Deserialize;

mod parsers;
pub mod term;

#[derive(Debug, Deserialize)]
pub struct Network {
    #[serde(rename = "BSSID")]
    pub bssid: String,
    #[serde(rename = "SSID")]
    pub ssid: String,
    #[serde(rename = "CHAN")]
    pub channel: u8,
    #[serde(rename = "SIGNAL")]
    pub signal: u8,
    #[serde(rename = "SECURITY", with = "parsers::network_security")]
    pub security: NetworkSecurity,
}

#[derive(Debug)]
pub struct NetworkSecurity(pub Security, pub Security);

#[derive(Debug)]
pub enum Security {
    WPA2,
    WPA1,
    _8021X,
    WEP,
    None,
}

impl From<String> for Security {
    fn from(string: String) -> Self {
        match string.as_str() {
            "WPA2" => Security::WPA2,
            "WPA1" => Security::WPA1,
            "_8021X" => Security::_8021X,
            "WEP" => Security::WEP,
            _ => Security::None,
        }
    }
}


