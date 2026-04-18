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
    let packages: Vec<Package> = pacman_results.lines()
        .filter(|line| !line.contains("-debug"))
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let name = tokens.next().unwrap().to_string();
            let version = tokens.next().unwrap().to_string();
            rpc_url += &format!("arg[]={}&", name);
            Package { name, version }
        })
        .collect();

    let multi_info: MultiInfo = reqwest::blocking::get(rpc_url)?.json()?;
    let query_results: HashMap<String, String> = multi_info.results.into_iter()
        .map(|info| (info.Name, info.Version))
        .collect();

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
