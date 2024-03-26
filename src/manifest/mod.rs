use crate::common::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Manifest {
    location: Location,
    commands: SchedulingDocument,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Location {
    kind: ShellKind,
    filter: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SchedulingDocument(Vec<SchedulingCommand>);

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SchedulingCommand {
    name: String,
    command: String,
    args: BTreeMap<String, String>,
    #[serde(rename = "schedule-with")]
    schedule_with: Option<Vec<String>>,
}

impl Manifest {
    pub fn from_io(io: impl std::io::Read) -> Result<Self> {
        Ok(serde_yaml::from_reader(io)?)
    }

    pub fn to_io(&self, io: impl std::io::Write) -> Result<()> {
        Ok(serde_yaml::to_writer(io, self)?)
    }

    pub fn from_file(filename: &Path) -> Result<Self> {
        let mut io = std::fs::OpenOptions::new();
        io.read(true);

        Self::from_io(io.open(filename)?)
    }

    pub fn to_file(&self, filename: &Path) -> Result<()> {
        let mut io = std::fs::OpenOptions::new();
        io.write(true);
        io.create(true);

        self.to_io(io.open(filename)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_document() -> Result<()> {
        let mut io = std::fs::OpenOptions::new();
        io.read(true);
        let loc: Location = serde_yaml::from_reader(io.open("testdata/location-one.yaml")?)?;
        assert_eq!(loc.kind, ShellKind::Systemd);
        let dc = loc.filter.get("datacenter");
        assert!(dc.is_some());
        assert_eq!(dc.unwrap(), "xo");
        Ok(())
    }

    #[test]
    fn test_scheduling_document() -> Result<()> {
        let mut io = std::fs::OpenOptions::new();
        io.read(true);
        let sched: SchedulingCommand =
            serde_yaml::from_reader(io.open("testdata/scheduling-one.yaml")?)?;
        assert_eq!(sched.name, "foo");
        assert_eq!(sched.command, "schedule");
        let kind = sched.args.get("kind");
        assert!(kind.is_some());
        assert_eq!(kind.unwrap(), "nspawn");
        let image = sched.args.get("image");
        assert!(image.is_some());
        assert_eq!(image.unwrap(), "nginx");
        assert!(sched.schedule_with.is_none());
        Ok(())
    }

    #[test]
    fn test_manifest() -> Result<()> {
        use std::str::FromStr;
        let manifest =
            Manifest::from_file(&std::path::PathBuf::from_str("testdata/combined-one.yaml")?)?;
        assert_eq!(manifest.location.kind, ShellKind::Systemd);
        let dc = manifest.location.filter.get("datacenter");
        assert!(dc.is_some());
        assert_eq!(dc.unwrap(), "xo");
        assert_eq!(manifest.commands.0[0].name, "foo-network");
        assert_eq!(manifest.commands.0[0].command, "network");
        let kind = manifest.commands.0[0].args.get("kind");
        assert!(kind.is_some());
        assert_eq!(kind.unwrap(), "veth");
        let ipv4 = manifest.commands.0[0].args.get("ipv4-props");
        assert!(ipv4.is_some());
        assert_eq!(ipv4.unwrap(), "address=192.168.1.1");
        let gateway = manifest.commands.0[0].args.get("gateway-phy");
        assert!(gateway.is_some());
        assert_eq!(gateway.unwrap(), "eth0");
        assert!(manifest.commands.0[0].schedule_with.is_some());
        assert_eq!(
            manifest.commands.0[0].schedule_with.clone().unwrap()[0],
            "foo"
        );
        assert_eq!(manifest.commands.0[1].name, "foo");
        assert_eq!(manifest.commands.0[1].command, "schedule");
        let kind = manifest.commands.0[1].args.get("kind");
        assert!(kind.is_some());
        assert_eq!(kind.unwrap(), "nspawn");
        let image = manifest.commands.0[1].args.get("image");
        assert!(image.is_some());
        assert_eq!(image.unwrap(), "nginx");
        assert!(manifest.commands.0[1].schedule_with.is_none());
        Ok(())
    }
}
