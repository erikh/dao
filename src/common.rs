use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ShellKind {
    #[serde(rename = "systemd")]
    Systemd,
}

impl std::fmt::Display for ShellKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Systemd => f.write_str("systemd"),
        }
    }
}

impl FromStr for ShellKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "systemd" => Ok(Self::Systemd),
            _ => Err(anyhow!("invalid shell kind '{}'", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemdKind {
    Timer,
    NSpawn,
    Machine,
    OneShot,
    Service,
}

impl std::fmt::Display for SystemdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Timer => "timer",
            Self::NSpawn => "nspawn",
            Self::Machine => "machine",
            Self::OneShot => "oneshot",
            Self::Service => "service",
        })
    }
}

impl FromStr for SystemdKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(match s {
            "timer" => Self::Timer,
            "nspawn" => Self::NSpawn,
            "machine" => Self::Machine,
            "oneshot" => Self::OneShot,
            "service" => Self::Service,
            _ => return Err(anyhow!("Invalid scheduling kind")),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Systemd(SystemdKind),
    Other,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Systemd(k) => f.write_str(&format!("{}", k)),
            Self::Other => f.write_str("other"),
        }
    }
}

impl FromStr for Kind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Self::Systemd(SystemdKind::from_str(s)?))
    }
}
