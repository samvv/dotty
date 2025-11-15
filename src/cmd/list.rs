
use clap::Parser;

use crate::Exec;

#[derive(Parser)]
pub struct ListCmd {

}

impl Exec for ListCmd {
    fn exec(&self, inv: &crate::Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        let index = repo.index()?;
        for entry in index.iter() {
            eprintln!("{}", String::from_utf8_lossy(&entry.path));
        }
        Ok(())
    }
}
