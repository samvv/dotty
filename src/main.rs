use std::{io::Write, path::{Path, PathBuf}};

use gethostname::gethostname;
use tera::Tera;
use clap::Parser;
use walkdir::WalkDir;

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
}

trait PathExt {
    fn join_inside<P: AsRef<Path>>(&self, other: P) -> Option<PathBuf>;
}

impl PathExt for PathBuf {
    fn join_inside<P: AsRef<Path>>(&self, other: P) -> Option<PathBuf> {
        let other = other.as_ref();
        if other.is_absolute() {
            let is_inside_self = pathdiff::diff_paths(other, self).is_some();
            if is_inside_self {
                Some(other.to_path_buf())
            } else {
                None // Or an error
            }
        } else {
            Some(self.join(other))
        }
    }
}

fn main() -> anyhow::Result<()> {

    let homedir = std::env::home_dir().expect("could not determine the home directory of the current user");

    let cli = Cli::parse();
    let user_mode = cli.user;
    let root_path = std::path::absolute(cli.root)
        .expect("failed to convert '--root' flag value to an absolute path");
    let source_path = root_path
        .join_inside(
            cli.source.unwrap_or(
                if user_mode {
                    homedir.join(".dotty")
                } else {
                    "config".into()
                }
            )
        )
        .expect(&format!("specified target path is not inside {}", root_path.display()));
    let target_path = root_path
        .join_inside(
            cli.target.unwrap_or(
                if user_mode {
                    homedir
                } else {
                    ".".into()
                }
            )
        )
        .expect(&format!("specified target path is not inside {}", root_path.display()));
    let hostname = cli.hostname.unwrap_or_else(|| gethostname().into_string().expect("failed to parse hostname of current device into UTF-8"));

    let mut tera = Tera::new(
    source_path
            .join("templates/**/*.tera")
            .to_str()
            .expect("received a source path with weird characters")
    )?;

    let mut ctx = tera::Context::new();
    ctx.insert("hostname", &hostname);

    let server_config_dir = source_path.join("servers").join(hostname);

    for result in WalkDir::new(&server_config_dir) {
        let entry = result?;
        let input_path = entry.path(); // MUST be relative to server_config_dir
        if !entry.file_type().is_file() {
            if !entry.file_type().is_dir() {
                eprintln!("Skipping non-regular file {}", input_path.to_string_lossy());
            }
            continue;
        }
        let rel_input_path = pathdiff::diff_paths(&input_path, &server_config_dir).unwrap();
        let ext_str = match input_path.extension().map(|x| x.to_str()) {
            None => None,
            Some(None) => {
                eprintln!("Failed to write {:?} because the path extensions contains invalid UTF-8 characters", input_path);
                continue;
            },
            Some(Some(s)) => Some(s),
        };
        match ext_str {
            Some("tera") => {
                let input = std::fs::read_to_string(input_path)?;
                let output = tera.render_str(&input, &ctx)?;
                let output_path = target_path.join(remove_extension(&rel_input_path));
                eprintln!("Writing {}", output_path.to_string_lossy());
                std::fs::create_dir_all(output_path.parent().unwrap())?; // FIXME?
                let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(&output_path)?;
                file.write_all(output.as_bytes())?;
            },
            _ => {
                let output_path = target_path.join(&rel_input_path);
                eprintln!("Writing {}", output_path.to_string_lossy());
                std::fs::create_dir_all(output_path.parent().unwrap())?;
                std::fs::copy(input_path, output_path)?;
            }
        }
    };

    Ok(())

}

fn remove_extension(path: &Path) -> PathBuf {
    let stripped = match path.file_stem() {
        Some(stem) => stem,
        None => return path.to_path_buf(),
    };
    match path.parent() {
        None => stripped.into(),
        Some(parent) => parent.join(stripped),
    }
}
