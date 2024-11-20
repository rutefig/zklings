use std::{path::Path, process::Command};

use anyhow::Result;

use crate::cmd::run_cmd;

pub struct SnarkjsCmd<'a> {
    pub args: &'a [&'a str],
    pub pot_dir: &'a Path,
    pub description: &'a str,
    /// The output buffer to append the merged stdout and stderr.
    pub output: &'a mut Vec<u8>,
}

impl<'a> SnarkjsCmd<'a> {
    pub fn run(&mut self) -> Result<bool> {
        let mut cmd = Command::new("snarkjs");
        cmd.current_dir(self.pot_dir).args(self.args);

        run_cmd(cmd, self.description, self.output)
    }
}
