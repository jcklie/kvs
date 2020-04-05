use std::process;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(no_version, about, author)]
struct Opt {
    /// Prints version information
    #[structopt(short = "V", long = "version")]
    version: bool,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    /// Get the string value of a given string key
    Get {
        #[structopt()]
        key: String,
    },

    /// Set the value of a string key to a string
    Set {
        #[structopt()]
        key: String,
        #[structopt()]
        value: String,
    },

    /// Remove a given key
    Rm {
        #[structopt()]
        key: String,
    },
}

fn main() {
    let opt: Opt = Opt::from_args();

    if let Some(cmd) = opt.cmd {
        match cmd {
            Command::Get { key } => handle_get(key),
            Command::Set { key, value } => handle_set(key, value),
            Command::Rm { key } => handle_rm(key),
        }
    } else if opt.version {
        handle_version()
    } else {
        process::exit(1);
    }
}

fn handle_get(_key: String) {
    eprintln!("unimplemented");
    process::exit(1);
}

fn handle_set(_key: String, _value: String) {
    eprintln!("unimplemented");
    process::exit(1);
}

fn handle_rm(_key: String) {
    eprintln!("unimplemented");
    process::exit(1);
}

fn handle_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("{:}", version)
}
