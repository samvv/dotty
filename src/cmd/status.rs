
use clap::Parser;
use git2::Delta;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct StatusCmd {
    #[arg(long, help = "Display all files currently being staged")]
    all: bool,
}

impl Exec for StatusCmd {
    fn exec(&self, inv: &Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        let index = repo.index()?;
        if self.all {
            for entry in index.iter() {
                eprintln!("{}", String::from_utf8_lossy(&entry.path));
            }
        } else {
            let head = repo.head()?.peel_to_commit()?;
            let diff = repo.diff_tree_to_index(Some(&head.tree()?), Some(&index), None)?;
            for delta in diff.deltas() {
                match delta.status() {
                    Delta::Added => eprintln!("A {}", delta.new_file().path().unwrap().display()),
                    Delta::Renamed => eprintln!("R {} -> {}", delta.old_file().path().unwrap().display(), delta.new_file().path().unwrap().display()),
                    Delta::Deleted => eprintln!("D {}", delta.old_file().path().unwrap().display()),
                    Delta::Copied => eprintln!("C {}", delta.new_file().path().unwrap().display()),
                    Delta::Ignored | Delta::Unmodified => {},
                    Delta::Modified | Delta::Typechange => eprintln!("M {}", delta.old_file().path().unwrap().display()),
                    Delta::Untracked => eprintln!("? {}", delta.old_file().path().unwrap().display()),
                    Delta::Unreadable => log::warn!("unable to read {}", delta.old_file().path().unwrap().display()),
                    Delta::Conflicted => eprintln!("U {}", delta.old_file().path().unwrap().display()),
                }
            }
        }
        Ok(())
    }
}
