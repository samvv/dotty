
use clap::Parser;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct InitCmd {
}

impl Exec for InitCmd {
    fn exec(&self, inv: &Invocation) -> anyhow::Result<()> {
        let _ = inv.repo();
        Ok(())
    }
}

