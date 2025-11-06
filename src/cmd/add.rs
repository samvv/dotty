
use std::path::PathBuf;

use clap::Parser;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct AddCmd {
    paths: Vec<PathBuf>,
}

impl Exec for AddCmd {
    fn exec(&self, _inv: &Invocation) -> anyhow::Result<()> {
        todo!()
    }
}
