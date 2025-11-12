
use std::process::Command;

use clap::Parser;
use git2::ErrorCode;

use crate::Exec;

#[derive(Parser)]
pub struct CommitCmd {
    #[arg(short, long, help = "The message to add to the commit")]
    message: Option<String>,
    #[arg(long, help = "Adjust the most recent commit")]
    amend: bool,
}

impl Exec for CommitCmd {
    fn exec(&self, inv: &crate::Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        let message = match &self.message {
            Some(text)  => text.clone(),
            None => inquire::Editor::new(&format!("Commit message: Update config for {}", inv.hostname))
                .with_predefined_text("\n# Lines starting with a '#' are ignored")
                .prompt()?,
        };
        if message.trim().len() == 0 {
            eprintln!("Operation cancelled due to empty commit message.");
            return Ok(());
        }
        let mut index = repo.index()?;
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
        let sig = repo.signature()?;
        let parent = match repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(error) if error.code() == ErrorCode::UnbornBranch => None,
            Err(error) => return Err(error.into()),
        };
        let mut parents = Vec::new();
        if let Some(commit) = parent.as_ref() {
            parents.push(commit);
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
