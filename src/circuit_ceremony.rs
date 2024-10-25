use anyhow::Result;

use std::io::Write;
use std::path::Path;

use crate::cmd_snarkjs::SnarkjsCmd;

pub struct CircuitCeremony<'a> {
    pub working_dir: &'a Path,
    pub ptau: &'a str,
}

impl<'a> CircuitCeremony<'a> {
    pub fn start_ceremony(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Start ceremony...")?;

        let mut start_ceremony_cmd = SnarkjsCmd {
            pot_dir: &self.working_dir,
            args: &[
                "powersoftau",
                "new",
                "bn128",
                &self.ptau,
                &self.contribution_file_0(),
                "-v",
            ],

            description: "Start power of tau ceremony",
            output,
        };

        start_ceremony_cmd.run()
    }

    pub fn contribute_ceremony(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Contribute to the ceremony...")?;

        // Create ceremony with skipping entropy input
        let mut contribute_ceremony_cmd = SnarkjsCmd {
            pot_dir: &self.working_dir,
            args: &[
                "ptc",
                &self.contribution_file_0(),
                &self.contribution_file_1(),
                "-v",
                "-e",
            ],
            description: "First contribution",
            output,
        };

        contribute_ceremony_cmd.run()
    }

    // File name as parameters for ceremony.
    pub fn contribution_file_0(&self) -> String {
        self.contribution_file_name("_0000")
    }

    pub fn contribution_file_1(&self) -> String {
        self.contribution_file_name("_0001")
    }

    pub fn contribution_file_final(&self) -> String {
        self.contribution_file_name("_final")
    }

    pub fn contribution_file_name(&self, suffix: &str) -> String {
        format!("pot{}{}.ptau", &self.ptau, &suffix)
    }
}
