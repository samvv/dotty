
use clap::Parser;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct StatusCmd {
    
}

impl Exec for StatusCmd {
    fn exec(&self, _inv: &Invocation) -> anyhow::Result<()> {
        todo!()
    }
}
