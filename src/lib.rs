pub mod common;
pub mod db;
pub mod manifest;
pub mod protocol;
pub mod transports;

#[cfg(test)]
pub(crate) mod testdata {
    use crate::common::*;
    use crate::protocol::*;
    use chrono::NaiveDateTime;
    use std::collections::HashMap;

    fn make_map(set: Vec<String>) -> HashMap<String, String> {
        let mut map = std::collections::HashMap::default();

        for item in &set {
            let parts = item
                .splitn(2, '=')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            map.insert(parts[0].clone(), parts[1].clone());
        }

        map
    }

    lazy_static::lazy_static! {
        pub(crate) static ref TAG_SETS: Vec<HashMap<String, String>> = vec![
            make_map(vec!["one=foo".to_string(), "two=bar".to_string()]),
            make_map(vec!["three=baz".to_string(), "four=quux".to_string()]),
            make_map(vec!["five=frobnik".to_string()])
        ];

        pub(crate) static ref GREEN_TABLE: Vec<(String, Instruction, String, Response, String, String)> = vec![
            (
                "schedule name=\"test\" image=\"linux\" kind=\"nspawn\" tags=\"one=foo,two=bar\"".into(),
                Instruction {
                    command: Command::Schedule(
                        "test".to_string(),
                        "linux".to_string(),
                        Kind::Systemd(SystemdKind::NSpawn),
                    ),
                    tags: TAG_SETS[0].clone(),
                },
                "schedule test".into(),
                Response {
                    status: true,
                    error: None,
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload: Default::default(),
                },
                "{\"status\":true,\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{}}".into(),
                "basic response".into(),
            ),
            (
                "terminate name=\"terminate-test\" tags=\"four=quux,three=baz\"".into(),
                Instruction {
                    command: Command::Terminate("terminate-test".to_string()),
                    tags: TAG_SETS[1].clone(),
                },
                "terminate test".into(),
                Response {
                    status: true,
                    error: None,
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload: Default::default(),
                },
                "{\"status\":true,\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{}}".into(),
                "basic response".into(),
            ),
            (
                "status name=\"status-test\" tags=\"five=frobnik\"".into(),
                Instruction {
                    command: Command::Status(Some("status-test".to_string())),
                    tags: TAG_SETS[2].clone(),
                },
                "status test w/ name".into(),
                Response {
                    status: true,
                    error: None,
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload: Default::default(),
                },
                "{\"status\":true,\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{}}".into(),
                "basic response".into(),
            ),
            (
                "status".into(),
                Instruction {
                    command: Command::Status(None),
                    tags: std::collections::HashMap::default(),
                },
                "status test w/o name".into(),
                Response {
                    status: true,
                    error: None,
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload: Default::default(),
                },
                "{\"status\":true,\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{}}".into(),
                "basic response".into(),
            ),
        ];

        pub(crate) static ref RED_TABLE: Vec<(String, String)> = vec![
            ("schedule".into(), "schedule with no keys".into()),
            ("terminate".into(), "terminate with no keys".into()),
            ("schedule name=\"blah\"".into(), "schedule with only name keys".into()),
            ("schedule kind=\"nspawn\"".into(), "schedule with only kind keys".into()),
            ("schedule image=\"linux\"".into(), "schedule with only image keys".into()),
            (
                "schedule kind=\"nspawn\" image=\"linux\"".into(),
                "schedule without name key".into(),
            ),
        ];
    }
}
