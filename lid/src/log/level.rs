use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
    Disabled,
    Error,
    Warning,
    Info,
    Debug,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Level::Disabled => "disabled",
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Info => "info",
            Level::Debug => "debug",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for Level {
    type Err = String;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        match l {
            "disabled" => Ok(Level::Disabled),
            "error" => Ok(Level::Error),
            "warning" => Ok(Level::Warning),
            "info" => Ok(Level::Info),
            "debug" => Ok(Level::Debug),
            _ => Err(format!("Invalid log level: {l}")),
        }
    }
}
