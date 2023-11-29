use serde_derive::Deserialize;
use std::fs::{self, File};
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
pub struct Package {
    pub remote: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Local {
    pub home: String,
}

#[derive(Deserialize, Debug)]
pub struct Install {
    pub steps: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Data {
    pub package: Package,
    pub local: Local,
    pub install: Install,
}

fn parse(file_path: &str) -> (File, String, Data) {
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read file {:?}", file_path);
            eprintln!("{}", e);
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from {:?}", file_path);
            exit(1);
        }
    };

    let script_file_name = format!("/home/tyler/RustroverProjects/winch/parsertest/install_{}.sh", data.package.name);
    let mut script_file = match File::create(&script_file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not create script file {:?}", script_file_name);
            eprintln!("{}", e);
            exit(1);
        }
    };

    return (script_file, script_file_name, data)
}