use serde::{Deserialize, Serialize};
use std::{fmt::Display, os::raw::c_ulong};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(untagged)]
pub enum Size {
    Pixel(i32),
    Ratio(f32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Tag,
    Workspace,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InsertBehavior {
    Top,
    #[default]
    Bottom,
    BeforeCurrent,
    AfterCurrent,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    #[default]
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Off => write!(f, "Off"),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Warn => write!(f, "Warn"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Trace => write!(f, "Trace"),
        }
    }
}

impl PartialEq<String> for LogLevel {
    fn eq(&self, other: &String) -> bool {
        other
            .parse::<usize>()
            .ok()
            .and_then(|num| match num {
                n @ 0..=5 => Some(n),
                _ => {
                    println!("Numeric log levels must be in 0..=5");
                    None
                }
            })
            .or_else(|| match other.as_str() {
                "" => Some(1),
                s if s.eq_ignore_ascii_case("error") => Some(1),
                s if s.eq_ignore_ascii_case("warn") => Some(2),
                s if s.eq_ignore_ascii_case("info") => Some(3),
                s if s.eq_ignore_ascii_case("debug") => Some(4),
                s if s.eq_ignore_ascii_case("trace") => Some(5),
                s if s.eq_ignore_ascii_case("off") => Some(0),
                _ => {
                    println!("Loglevel name must be one of \"error\", \"warn\", \"info\", \"debug\", \"trace\" or \"off\" (Case insensitive) ");
                    None
                },
            }).map(|l| *self as usize == l).unwrap_or(false)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FocusBehaviour {
    Sloppy,
    ClickTo,
    Driven,
}

pub type Window = c_ulong;
type MockHandle = i32;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowHandle {
    MockHandle(MockHandle),
    XlibHandle(Window),
}
