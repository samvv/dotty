
mod util;
mod cmd;

use std::path::PathBuf;

use gethostname::gethostname;
use clap::{Parser, Subcommand};

use crate::util::PathExt;
use crate::cmd::{AddCmd, InitCmd, StatusCmd, UnpackCmd};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, help = "Configure the home directory instead of the system")]
    user: bool,
    #[arg(short, long, default_value = "/", help = "Act as if chroot-ing into this directory")]
    root: PathBuf,
    #[arg(short, long, help = "The path to the original configuration files")]
    source: Option<PathBuf>,
    #[arg(short, long, help = "Where to write the configuration files to")]
    target: Option<PathBuf>,
    #[arg(long, help = "Select anther hostname than that of the current machine")]
    hostname: Option<String>,
    #[command(subcommand)]
    command: Command,
    #[clap(long, help = "Do not ask for confirmations")]
    force: bool,
}

#[derive(Subcommand)]
enum Command {
    Add(AddCmd),
    Init(InitCmd),
    Status(StatusCmd),
    Unpack(UnpackCmd),
}

impl Exec for Command {
    fn exec(&self, inv: &Invocation) -> anyhow::Result<()> {
        match self {
            Command::Add(inner) => inner.exec(&inv),
            Command::Init(inner) => inner.exec(&inv),
            Command::Status(inner) => inner.exec(&inv),
            Command::Unpack(inner) => inner.exec(&inv),
        }
    }
}

pub trait Exec {
    fn exec(&self, inv: &Invocation) -> anyhow::Result<()>;
}

pub struct Invocation {
    user_mode: bool,
    root_path: PathBuf,
    source_path: PathBuf,
    target_path: PathBuf,
    hostname: String,
    force: bool,
}

fn main() -> anyhow::Result<()> {

    env_logger::init();

    let homedir = std::env::home_dir().expect("could not determine the home directory of the current user");

    let cli = Cli::parse();
    let force = cli.force;
    let user_mode = cli.user;
    let root_path = std::path::absolute(cli.root)
        .expect("failed to convert '--root' flag value to an absolute path");
    let source_path = std::path::absolute(
        root_path
        .join_inside(
            cli.source.unwrap_or(
                if user_mode {
                    homedir.join(".dotty")
                } else {
                    "config".into()
                }
            )
        )
        .expect(&format!("specified target path is not inside {}", root_path.display()))
    ).unwrap();
    let target_path = std::path::absolute(
        root_path
            .join_inside(
                cli.target.unwrap_or(
                    if user_mode {
                        homedir
                    } else {
                        ".".into()
                    }
                )
            )
            .expect(&format!("specified target path is not inside {}", root_path.display()))
    ).unwrap();
    let hostname = cli.hostname.unwrap_or_else(|| gethostname().into_string().expect("failed to parse hostname of current device into UTF-8"));

    let inv = Invocation {
        hostname,
        user_mode,
        root_path,
        source_path,
        target_path,
        force,
    };

    cli.command.exec(&inv)
}

