use serde::Deserialize;

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
    #[serde(rename = "SECURITY", with = "network_security_parser")]
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

mod network_security_parser {
    use serde::{de::Error, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(d: D) -> Result<super::NetworkSecurity, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(string) => {
                let mut iter = string.split_whitespace();
                Ok(super::NetworkSecurity(
                    match iter.next() {
                        None => super::Security::None,
                        Some(security_str) => super::Security::from(security_str.to_string()),
                    },
                    match iter.next() {
                        None => super::Security::None,
                        Some(security_str) => super::Security::from(security_str.to_string()),
                    },
                ))
            }
            _ => Err(D::Error::custom(format_args!(
                "nmcli data malformed or unsupported"
            ))),
        }
    }
}
