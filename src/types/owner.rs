/// Owner of a screen cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Owner {
    Monitor,
    Window,
    Region,
}

impl std::str::FromStr for Owner {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "monitor" => Ok(Self::Monitor),
            "window" => Ok(Self::Window),
            "region" => Ok(Self::Region),
            _ => Err(()),
        }
    }
}
