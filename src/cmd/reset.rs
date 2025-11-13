
use std::path::PathBuf;

use clap::Parser;

use crate::Exec;

#[derive(Parser)]
pub struct ResetCmd {
    #[arg(help = "Only reset the specified paths if provided")]
    paths: Vec<PathBuf>,
}

impl Exec for ResetCmd {
    fn exec(&self, inv: &crate::Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        let mut index = repo.index()?;
        if self.paths.is_empty() {
            match repo.head() {
                Ok(head) => {
                    let tree = head.peel_to_commit()?.tree()?;
                    index.read_tree(&tree)?;
                    index.write()?;
                    // repo.reset(commit.as_object(), git2::ResetType::Mixed, None)?;
                }
                Err(error) if error.code() == git2::ErrorCode::UnbornBranch => {
                    index.clear()?;
                    index.write()?;
                }
                Err(error) => return Err(error.into()),
            };
        } else {
            todo!()
        }
        Ok(())
    }
}
