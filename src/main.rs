use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    cmd: Option<Command>,
}

#[derive(Parser, Debug)]
enum Command {
    Install {
        #[clap()]
        package: String,

        #[clap(long, short = 'f')]
        force: bool,

        #[clap(long, short = 'r')]
        remote_repo: bool,
    },
    Uninstall {
        #[clap()]
        package: String,

        #[clap(long, short = 'f')]
        force: bool,

        #[clap(long, short = 'r')]
        remote_repo: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Some(Command::Install { package, force, remote_repo}) => {
            println!("Install command with package: {}", package);
            println!("force: {}, remote_repo: {}", force, remote_repo);
        }
        Some(Command::Uninstall { package, force, remote_repo}) => {
            println!("Uninstall command with package: {}", package);
            println!("force: {}, remote_repo: {}", force, remote_repo);
        }
        None => {
            println!("No subcommand provided");
        }
    }
}