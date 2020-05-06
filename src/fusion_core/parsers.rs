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
                        Some(security_str) => {
                            crate::fusion_core::Security::from(security_str.to_string())
                        }
                    },
                    match iter.next() {
                        None => crate::fusion_core::Security::None,
                        Some(security_str) => {
                            crate::fusion_core::Security::from(security_str.to_string())
                        }
                    },
                ))
            }
            _ => Err(D::Error::custom(format_args!(
                "nmcli data malformed or unsupported"
            ))),
        }
    }
}

pub mod frequency {
    use serde::{de::Error, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(d: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(string) => {
                let value = string.split_whitespace().next().unwrap();
                Ok(value.parse().unwrap())
            }
            _ => Err(D::Error::custom(format_args!(
                "nmcli data malformed or unsupported"
            ))),
        }
    }
}
