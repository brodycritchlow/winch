use std::fs::File;
use std::io::Write;
use crate::config_parser::Data;

pub fn create_install_script_file(script_file: &mut File, script_file_name: String, data: Data){
    #[cfg(target_os = "linux")]
    let bin_bash = "#!/bin/bash";

    #[cfg(not(target_os = "linux"))]
    let bin_bash = "";

    #[cfg(target_os = "linux")]
    let and = "&&";

    #[cfg(not(target_os = "linux"))]
    let and = ";";

    writeln!(script_file, "{}", bin_bash).expect("Error writing to script file");
    writeln!(script_file, "cd {} || mkdir ~/winch {} cd ~/winch" , data.local.home, or, and).expect("Error writing to script file");

    // Git clone command
    writeln!(
        script_file,
        "echo 'Cloning repository: {}'",
        data.package.remote
    )
        .expect("Error writing to script file");
    writeln!(
        script_file,
        "git clone {} {} exit 1",
        data.package.remote, and
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