use crate::common::*;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub command: Command,
    pub tags: HashMap<String, String>,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags = if self.tags.is_empty() {
            Default::default()
        } else {
            let mut tags = self
                .tags
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>();

            tags.sort();

            format!(r#" tags="{}""#, tags.join(","))
        };
        match &self.command {
            Command::Schedule(name, image, kind) => f.write_str(&format!(
                r#"schedule name="{}" image="{}" kind="{}"{}"#,
                name, image, kind, tags,
            )),
            Command::Terminate(name) => {
                f.write_str(&format!(r#"terminate name="{}"{}"#, name, tags))
            }
            Command::Status(name) => f.write_str(&format!(
                "status{}{}",
                name.to_owned()
                    .map_or_else(|| Default::default(), |x| format!(r#" name="{}""#, x)),
                tags,
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Schedule(String, String, Kind),
    Terminate(String),
    Status(Option<String>),
}

lazy_static::lazy_static! {
    static ref PARSE_INSTRUCTION: regex::Regex =
        regex::Regex::new(r#"^\s*([^\s]+)\s*(.*)$"#).unwrap();
    static ref PARSE_KV_PAIR: regex::Regex =
        regex::Regex::new(r#"([^=\s]+)\s*=\s*\"(\\"|[^\"]+)\""#).unwrap();
}

fn parse_tags(tags: &str) -> Result<HashMap<String, String>> {
    let mut map = HashMap::default();

    for tag in tags.split(',') {
        let parts = tag
            .splitn(2, '=')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if parts.len() != 2 {
            return Err(anyhow!("invalid key=value pair in tags"));
        }

        map.insert(parts[0].clone(), parts[1].clone());
    }

    Ok(map)
}

fn parse_name_only(pairs: Vec<(String, String)>) -> Result<(String, HashMap<String, String>)> {
    let mut name = String::new();
    let mut tags = HashMap::default();

    for pair in pairs {
        let (key, value) = pair;
        match key.to_lowercase().as_str() {
            "name" => name = value,
            "tags" => tags = parse_tags(&value)?,
            _ => return Err(anyhow!("invalid argument in schedule command")),
        }
    }
    Ok((name, tags))
}

fn parse_kv_pairs(mut pairs: &str) -> Result<Vec<(String, String)>> {
    let mut v = Vec::new();

    while let Some(captures) = PARSE_KV_PAIR.captures(pairs) {
        let key = captures.get(1).unwrap();
        let value = captures.get(2).unwrap();

        v.push((key.as_str().to_string(), value.as_str().to_string()));
        let m = captures.get(0).unwrap();
        pairs = &pairs[m.end()..]
    }

    Ok(v)
}

impl Instruction {
    fn parse_status(pairs: &str) -> Result<Self> {
        let (name, tags) = parse_name_only(parse_kv_pairs(pairs)?)?;

        Ok(Self {
            command: Command::Status(if name.is_empty() { None } else { Some(name) }),
            tags,
        })
    }

    fn parse_terminate(pairs: &str) -> Result<Self> {
        let (name, tags) = parse_name_only(parse_kv_pairs(pairs)?)?;

        if name.is_empty() {
            return Err(anyhow!("name cannot be omitted"));
        }

        Ok(Self {
            command: Command::Terminate(name),
            tags,
        })
    }

    fn parse_schedule(pairs: &str) -> Result<Self> {
        let mut name = String::new();
        let mut image = String::new();
        let mut kind = String::new();
        let mut tags = HashMap::default();

        let pairs = parse_kv_pairs(pairs)?;

        for pair in pairs {
            let (key, value) = pair;

            match key.as_str().to_lowercase().as_str() {
                "name" => name = value.as_str().to_string(),
                "image" => image = value.as_str().to_string(),
                "kind" => kind = value.as_str().to_string(),
                "tags" => tags = parse_tags(value.as_str())?,
                _ => return Err(anyhow!("invalid argument in schedule command")),
            }
        }

        if name.is_empty() {
            return Err(anyhow!("name cannot be omitted"));
        }

        if image.is_empty() {
            return Err(anyhow!("image cannot be omitted"));
        }

        if kind.is_empty() {
            return Err(anyhow!("kind cannot be omitted"));
        }

        Ok(Self {
            command: Command::Schedule(name, image, Kind::from_str(&kind)?),
            tags,
        })
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if let Some(captures) = PARSE_INSTRUCTION.captures(s) {
            match captures.get(1).unwrap().as_str().to_lowercase().as_str() {
                "schedule" => Self::parse_schedule(captures.get(2).unwrap().as_str()),
                "terminate" => Self::parse_terminate(captures.get(2).unwrap().as_str()),
                "status" => Self::parse_status(captures.get(2).unwrap().as_str()),
                x => Err(anyhow!("invalid command in request: {:?}", x)),
            }
        } else {
            Err(anyhow!("no command specified. Input: {:?}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::*;

    #[test]
    fn test_display_methods() -> Result<()> {
        for (text, instruction, annotation, _, _, _) in &*GREEN_TABLE {
            assert_eq!(instruction.to_string(), *text, "{}", annotation);
        }

        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        for (text, result, annotation, _, _, _) in &*GREEN_TABLE {
            assert_eq!(text.parse::<Instruction>()?, *result, "{}", annotation);
        }

        for (text, annotation) in &*RED_TABLE {
            assert!(text.parse::<Instruction>().is_err(), "{}", annotation);
        }

        Ok(())
    }
}
