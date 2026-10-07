/// The address of a window, a unique hex handle to identify it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowAddress(u64);

impl std::str::FromStr for WindowAddress {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix("0x").unwrap_or(s);
        u64::from_str_radix(hex, 16).map(Self)
    }
}

impl<'de> serde::Deserialize<'de> for WindowAddress {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for WindowAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
