use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Neofetch,
    Splash,
}

impl Mode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" => Some(Mode::Default),
            "neofetch" => Some(Mode::Neofetch),
            "splash" | "boot" => Some(Mode::Splash),
            _ => None,
        }
    }

    pub fn default_fields(&self) -> Vec<String> {
        match self {
            Mode::Default => vec![
                "OS".into(),
                "Kernel".into(),
                "Uptime".into(),
                "Shell".into(),
                "CPU".into(),
                "Memory".into(),
            ],
            Mode::Neofetch => Config::default_fields(),
            Mode::Splash => vec![],
        }
    }

    pub fn default_palette(&self) -> bool {
        match self {
            Mode::Default => false,
            Mode::Neofetch => true,
            Mode::Splash => false,
        }
    }
}
