
use clap::Parser;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct StatusCmd {
    
}

impl Exec for StatusCmd {
    fn exec(&self, inv: &Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        let index = repo.index()?;
        for entry in index.iter() {
            eprintln!("{} (0x{:03o})", String::from_utf8_lossy(&entry.path), entry.mode);
        }
        Ok(())
    }
}
