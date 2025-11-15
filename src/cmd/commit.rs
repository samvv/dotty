
use std::process::Command;

use clap::Parser;
use git2::ErrorCode;

use crate::Exec;

#[derive(Parser)]
pub struct CommitCmd {
    #[arg(short, long, help = "The message to add to the commit")]
    message: Option<String>,
    #[arg(short, long, help = "Adjust the most recent commit")]
    amend: bool,
}

impl Exec for CommitCmd {
    fn exec(&self, inv: &crate::Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        let mut index = repo.index()?;
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
        let sig = repo.signature()?;
        let parent = match repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(error) if error.code() == ErrorCode::UnbornBranch => None,
            Err(error) => return Err(error.into()),
        };
        let is_empty = match &parent {
            Some(commit) => repo.diff_tree_to_index(
                Some(&commit.tree()?),
                Some(&index),
                None
            )?.deltas().len() == 0,
            None => index.is_empty(),
        };
        if is_empty {
            eprintln!("No changes to commit.");
            std::process::exit(1);
        }
        let mut parents = Vec::new();
        if let Some(commit) = parent.as_ref() {
            parents.push(commit);
        }
        let message = match &self.message {
            Some(text)  => text.clone(),
            None => {
                let short_msg = format!("Update config for {}", inv.hostname);
                inquire::Editor::new(&format!("Commit message: {}", short_msg))
                    .with_predefined_text(&format!("{}\n# Lines starting with a '#' are ignored", short_msg))
                    .prompt()?
            }
        };
        // FIXME ignore lines starting with '#'
        if message.trim().len() == 0 {
            eprintln!("Aborting commit due to empty commit message.");
            std::process::exit(1);
        }
        let commit = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &message,
            &tree,
            &parents,
        )?;
        eprintln!("Created commit {}", commit);
        Ok(())
    }
}
