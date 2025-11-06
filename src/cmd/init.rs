
use clap::Parser;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct InitCmd {
}

impl Exec for InitCmd {
    fn exec(&self, _inv: &Invocation) -> anyhow::Result<()> {
        todo!()
    }
}

