use std::convert::TryFrom;
use std::fmt;
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AppProfile {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "dev")]
    Dev,
    #[serde(rename = "prod")]
    Prod,
}

impl fmt::Display for AppProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppProfile::Test => write!(f, "test"),
            AppProfile::Dev => write!(f, "dev"),
            AppProfile::Prod => write!(f, "prod"),
        }
    }
}

impl TryFrom<&str> for AppProfile {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "test" => Ok(Self::Test),
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{other} is not a supported environment. Use either `dev` or `prod` or `test`."
            )),
        }
    }
}

impl TryFrom<String> for AppProfile {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AppProfile::try_from(&*value)
    }
}
