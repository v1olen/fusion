pub mod network_security {
    use serde::{de::Error, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(d: D) -> Result<crate::fusion_core::NetworkSecurity, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(string) => {
                let mut iter = string.split_whitespace();
                Ok(crate::fusion_core::NetworkSecurity(
                    match iter.next() {
                        None => crate::fusion_core::Security::None,
                        Some(security_str) => crate::fusion_core::Security::from(security_str.to_string()),
                    },
                    match iter.next() {
                        None => crate::fusion_core::Security::None,
                        Some(security_str) => crate::fusion_core::Security::from(security_str.to_string()),
                    },
                ))
            }
            _ => Err(D::Error::custom(format_args!(
                "nmcli data malformed or unsupported"
            ))),
        }
    }
}
