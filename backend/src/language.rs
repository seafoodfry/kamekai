use chrono::{Local, Timelike};
use clap::ValueEnum;
use std::fmt;

// First, we'll define our Language enum.
// We derive several useful traits:
// - Clone and Copy make it easy to pass around
// - Debug for printing during development
// - ValueEnum allows Clap to parse it from command line arguments
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Language {
    Japanese,
    Chinese,
}

// We implement Display to allow the conversion of the enum to a string.
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match on self to provide the string representation we want
        match self {
            Language::Japanese => write!(f, "japanese"),
            Language::Chinese => write!(f, "chinese"),
        }
    }
}

impl Language {
    pub fn get_greeting(&self) -> &'static str {
        match self {
            Language::Japanese => Self::get_japanese_greeting(),
            Language::Chinese => Self::get_chinese_greeting(),
        }
    }

    fn get_japanese_greeting() -> &'static str {
        match Local::now().hour() {
            5..=10 => "おはようございます。",
            11..=17 => "こんにちは。",
            _ => "こんばんは。",
        }
    }

    fn get_chinese_greeting() -> &'static str {
        match Local::now().hour() {
            5..=10 => "早上好。",
            11..=17 => "下午好。",
            _ => "晚上好。",
        }
    }
}
