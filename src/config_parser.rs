use serde_derive::Deserialize;
use std::fs::{self, File};
use std::process::exit;
use toml;
use std::env::consts::OS;
use reqwest;

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
pub struct Data {
    pub package: Package,
    pub local: Local,
    pub install: Install,
}

pub fn parse(url: &str) -> (File, String, Data) {
    println!("{}", url);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("{}", url))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "Awesome-Octocat-App")
        .header("Authorization", format!("Bearer {}", "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"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send();

    // let response = match get(url) {
    //     Ok(resp) => resp,
    //     Err(e) => {
    //         eprintln!("Failed to fetch data from URL: {}", url);
    //         eprintln!("{}", e);
    //         exit(1);
    //     }
    // };
    
    let binding = response.unwrap().text().unwrap();
    let contents = binding.as_str();
    println!("{}", contents);

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to parse data from the response, {}", e);
            exit(1);
        }
    };

    #[cfg(target_os = "linux")]
    let script_file_name = format!("./install.sh");

    #[cfg(target_os = "windows")]
    let script_file_name = format!("./install.cmd");

    let script_file = match File::create(&script_file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not create script file {:?}", script_file_name);
            eprintln!("{}", e);
            exit(1);
        }
    };

    return (script_file, script_file_name, data);
}