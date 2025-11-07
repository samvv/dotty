
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Parser;
use tera::Tera;
use walkdir::WalkDir;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct UnpackCmd {
    #[arg(short, long, help = "Do not ask for confirmation when overwriting files")]
    force: bool,
}

impl Exec for UnpackCmd {
    fn exec(&self, inv: &Invocation) -> anyhow::Result<()> {
        let mut tera = Tera::new(
        inv.source_path
                .join("templates/**/*.tera")
                .to_str()
                .expect("received a source path with weird characters")
        )?;

        let mut ctx = tera::Context::new();
        ctx.insert("hostname", &inv.hostname);

        let server_config_dir = inv.source_path.join("servers").join(&inv.hostname);

        for result in WalkDir::new(&server_config_dir) {
            let entry = result?;
            let input_path = entry.path(); // MUST be relative to server_config_dir
            if !entry.file_type().is_file() {
                if !entry.file_type().is_dir() {
                    log::warn!("Skipping non-regular file {}", input_path.to_string_lossy());
                }
                continue;
            }
            let rel_input_path = pathdiff::diff_paths(&input_path, &server_config_dir).unwrap();
            let ext_str = match input_path.extension().map(|x| x.to_str()) {
                None => None,
                Some(None) => {
                    log::error!("Failed to write {:?} because the path extensions contains invalid UTF-8 characters", input_path);
                    continue;
                },
                Some(Some(s)) => Some(s),
            };
            match ext_str {
                Some("tera") => {
                    let input = std::fs::read_to_string(input_path)?;
                    let output = tera.render_str(&input, &ctx)?;
                    let output_path = inv.target_path.join(remove_extension(&rel_input_path));
                    log::info!("Writing {}", output_path.to_string_lossy());
                    std::fs::create_dir_all(output_path.parent().unwrap())?; // FIXME?
                    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(&output_path)?;
                    file.write_all(output.as_bytes())?;
                },
                _ => {
                    let output_path = inv.target_path.join(&rel_input_path);
                    log::info!("Writing {}", output_path.to_string_lossy());
                    std::fs::create_dir_all(output_path.parent().unwrap())?;
                    std::fs::copy(input_path, output_path)?;
                }
            }
        };

        Ok(())
    }
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
