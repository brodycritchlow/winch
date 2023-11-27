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

        #[clap(long)]
        force: bool,

        #[clap(short)]
        f: bool,

        #[clap(long)]
        remote_repo: bool,

        #[clap(short)]
        rp: bool,
    },
    Uninstall {
        #[clap()]
        package: String,

        #[clap(long)]
        force: bool,

        #[clap(short)]
        f: bool,

        #[clap(long)]
        remote_repo: bool,

        #[clap(short)]
        rp: bool,
    },
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
