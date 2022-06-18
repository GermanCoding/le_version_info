use chrono::{DateTime, SubsecRound, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::process::exit;

const HELP: &str = "\
le_version_info

USAGE:
  le_version_info <file> <directory>

ARGS:
  <file> <PATH>         Filename to store data in (format is json)
  <directory> <URL>     URL to fetch build information from, e.g.
                        https://acme-v02.api.letsencrypt.org/build
";

const INFO: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " +",
    env!("CARGO_PKG_REPOSITORY")
);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
struct BuildInfo {
    build: String,
    first_seen: Option<DateTime<Utc>>,
    last_seen: Option<DateTime<Utc>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("{HELP}");
        exit(0);
    }
    let file_name = args.get(1).unwrap();
    let url = args.get(2).unwrap();
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(INFO)
        .build()?;

    let now = Utc::now().trunc_subsecs(0);
    let file = OpenOptions::new().read(true).open(&file_name)?;
    let reader = BufReader::new(file);
    let mut build_list: Vec<BuildInfo> = serde_json::from_reader(reader)?;
    let mut search_build = client
        .get(url)
        .send()?
        .error_for_status()?
        .text()?
        .trim()
        .replacen("Boulder=(", "", 1)
        .trim()
        .to_string();
    if search_build.ends_with(")") {
        search_build.pop();
    }

    let current = match build_list
        .iter_mut()
        .enumerate()
        .rfind(|(_, candidate)| candidate.last_seen == None)
    {
        None => None,
        Some((index, _)) => Some(index),
    };
    let (new_index, _) = match build_list
        .iter_mut()
        .enumerate()
        .rfind(|(_, candidate)| candidate.build.eq(&search_build))
    {
        None => {
            let new = BuildInfo {
                build: search_build,
                first_seen: Some(now.clone()),
                last_seen: None,
            };
            build_list.push(new);
            let index = build_list.len() - 1;
            (index, build_list.get_mut(index).unwrap())
        }
        Some((index, candidate)) => {
            candidate.last_seen = None;
            (index, candidate)
        }
    };
    if let Some(current_index) = current {
        if current_index != new_index {
            let mut old_build = build_list.get_mut(current_index).unwrap();
            old_build.last_seen = Some(now.clone());
        }
    }
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_name)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &build_list)?;
    Ok(())
}
