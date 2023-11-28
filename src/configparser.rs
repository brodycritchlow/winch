use serde_derive::Deserialize;
use std::fs::{self, File};
use std::io::Write;
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
struct Package {
    remote: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Local {
    home: String,
}

#[derive(Deserialize, Debug)]
struct Install {
    steps: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Data {
    package: Package,
    local: Local,
    install: Install,
}

fn main() {
    let file_path = "/home/tyler/RustroverProjects/untitled/config.winch";
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

    writeln!(script_file, "#!/bin/bash").expect("Error writing to script file");
    writeln!(script_file, "cd {} || mkdir ~/winch && cd ~/winch" , data.local.home).expect("Error writing to script file");

    // Git clone command
    writeln!(
        script_file,
        "echo 'Cloning repository: {}'",
        data.package.remote
    )
        .expect("Error writing to script file");
    writeln!(
        script_file,
        "git clone {} || exit 1",
        data.package.remote
    )
        .expect("Error writing to script file");
    writeln!(script_file, "cd {}", data.package.name).expect("Error writing to script file");
    writeln!(script_file, "").expect("Error writing to script file");

    for (idx, step) in data.install.steps.iter().enumerate() {
        writeln!(script_file, "echo 'Running step {}'", idx + 1).expect("Error writing to script file");
        writeln!(script_file, "{} || exit 1", step).expect("Error writing to script file");
        writeln!(script_file, "").expect("Error writing to script file");
    }

    println!("Script file created at: {:?}", script_file_name);
}