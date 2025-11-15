use std::{os::unix::process::CommandExt, process::Command};

use clap::Parser;

use crate::Exec;

#[derive(Parser)]
pub struct LogCmd {
    #[arg(trailing_var_arg = true, num_args = 0.., allow_hyphen_values = true)]
    flags: Vec<String>,
}

impl Exec for LogCmd {
    fn exec(&self, inv: &crate::Invocation) -> anyhow::Result<()> {
        Err(Command::new("git")
            .arg("--git-dir")
            .arg(&inv.git_dir)
            .arg("log")
            .args(&self.flags)
            .exec()
            .into())
    }
}
