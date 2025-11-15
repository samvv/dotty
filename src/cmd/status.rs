
use clap::Parser;

use colored::Colorize;
use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct StatusCmd {
}

impl Exec for StatusCmd {
    fn exec(&self, inv: &Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        let index = repo.index()?;
        let old_tree = match repo.head() {
            Ok(commit) => Some(&commit.peel_to_commit()?.tree()?),
            Err(error) if error.code() == git2::ErrorCode::UnbornBranch => None,
            Err(error) => return Err(error.into()),
        };
        let diff = repo.diff_tree_to_index(old_tree, Some(&index), None)?;
        for delta in diff.deltas() {
            match delta.status() {
                git2::Delta::Added => eprintln!("{} {}", " A".green(), delta.new_file().path().unwrap().display()),
                git2::Delta::Renamed => eprintln!(" R {} -> {}", delta.old_file().path().unwrap().display(), delta.new_file().path().unwrap().display()),
                git2::Delta::Deleted => eprintln!(" D {}", delta.old_file().path().unwrap().display()),
                git2::Delta::Copied => eprintln!(" C {}", delta.new_file().path().unwrap().display()),
                git2::Delta::Ignored => eprintln!("!! {}", delta.old_file().path().unwrap().display()),
                git2::Delta::Modified | git2::Delta::Typechange => eprintln!("{} {}", " M".red(), delta.old_file().path().unwrap().display()),
                git2::Delta::Untracked => eprintln!("?? {}", delta.old_file().path().unwrap().display()),
                git2::Delta::Unreadable => log::warn!("unable to read {}", delta.old_file().path().unwrap().display()),
                git2::Delta::Conflicted => eprintln!("UU {}", delta.old_file().path().unwrap().display()),
                git2::Delta::Unmodified => {},
            }
        }
        Ok(())
    }
}
