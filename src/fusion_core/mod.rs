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
    #[serde(rename = "FREQ", with = "parsers::frequency")]
    pub frequency: u16,
    #[serde(rename = "SIGNAL")]
    pub signal: u8,
    #[serde(rename = "SECURITY", with = "parsers::network_security")]
    pub security: NetworkSecurity,
}

impl Network {
    pub fn is_secured(&self) -> bool {
        match self.security {
            NetworkSecurity(Security::None, Security::None) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkSecurity(pub Security, pub Security);

impl std::fmt::Display for NetworkSecurity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub enum Security {
    WPA2,
    WPA1,
    _8021X,
    WEP,
    None,
}

impl std::fmt::Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Security::WPA2 => "WPA2",
                Security::WPA1 => "WPA1",
                Security::_8021X => "802.1X",
                Security::WEP => "WEP",
                Security::None => "",
            }
        )
    }
}

impl From<String> for Security {
    fn from(string: String) -> Self {
        match string.as_str() {
            "WPA2" => Security::WPA2,
            "WPA1" => Security::WPA1,
            "802.1X" => Security::_8021X,
            "WEP" => Security::WEP,
            _ => Security::None,
        }
    }
}
