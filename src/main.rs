#![allow(non_snake_case)]

use std::collections::HashMap;
use std::process::Command;
use serde::Deserialize;
use version_compare::Cmp;

struct Package {
    name: String,
    version: String
}

#[derive(Deserialize)]
struct MultiInfo {
    resultcount: usize,
    results: Vec<Info>
}

#[derive(Deserialize)]
struct Info {
    Name: String,
    Version: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pacman_output = Command::new("pacman").arg("-Qme").output()?;
    let pacman_results = String::from_utf8(pacman_output.stdout)?;

    let mut rpc_url = String::from("https://aur.archlinux.org/rpc/v5/info?");
    let mut packages = Vec::new();
    for line in pacman_results.lines() {
        let mut tokens = line.split_whitespace();
        let name = tokens.next().unwrap();
        let version = tokens.next().unwrap();
        if !name.ends_with("-debug") {
            rpc_url += &format!("arg[]={}&", name);
            packages.push(Package {
                name: name.to_string(),
                version: version.to_string()
            });
        }
    }

    let multi_info: MultiInfo = reqwest::blocking::get(rpc_url)?.json()?;
    let mut query_results = HashMap::with_capacity(multi_info.resultcount);
    for info in multi_info.results {
        query_results.insert(info.Name, info.Version);
    }

    for package in packages {
        if let Some(aur_version) = query_results.get(&package.name) {
            match version_compare::compare(&package.version, aur_version) {
                Ok(Cmp::Lt) => {
                    println!("{} {} -> {}", package.name, package.version, aur_version);
                }
                _ => {}
            }
        } else {
            println!("{} {} -> Not Found", package.name, package.version);
        }
    }

    Ok(())
}
