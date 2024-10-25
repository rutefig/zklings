use anyhow::{Ok, Result};

use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io::Write};

use crossterm::style::Stylize;

use crate::path::change_extension_with_suffix;
use crate::{
    cmd::WasmWitnessCmd,
    cmd_snarkjs::SnarkjsCmd,
    path::{append_compiled_folder, change_extension},
};

pub struct CircomExercise<'a> {
    pub circuit_dir: &'a Path,
    pub circuit_file: &'a OsStr,
}

impl<'a> CircomExercise<'a> {
    pub fn generate_witness(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Computing witness...".underlined())?;

        let compiled_dir = append_compiled_folder(self.circuit_dir, &self.compiled_folder());
        let wasm_file = change_extension(self.circuit_file, "wasm");

        let mut generate_witness_cmd = WasmWitnessCmd {
            args: &[
                &wasm_file.display().to_string(),
                &self.input_file_dir(),
                "witness.wtns",
            ],
            description: "Computing witness",
            output,
            compiled_dir: &compiled_dir,
        };

        generate_witness_cmd.run()
    }

    pub fn prepare_circuit_proof(
        &self,
        output: &mut Vec<u8>,
        contribution_file_1: String,
        contribution_file_final: String,
    ) -> Result<bool> {
        writeln!(output, "{}", "Prepare circuit...")?;

        let mut prepare_circuit_proof_cmd = SnarkjsCmd {
            pot_dir: &self.circuit_dir,
            args: &[
                "pt2",
                &contribution_file_1,
                &contribution_file_final,
                "-v",
                "-e",
            ],
            description: "Prepare circuit-specific",
            output,
        };

        prepare_circuit_proof_cmd.run()
    }

    pub fn create_z_key(
        &self,
        output: &mut Vec<u8>,
        contribution_file_final: String,
    ) -> Result<bool> {
        writeln!(output, "{}", "Create .zkey ...")?;

        let r1cs_file = change_extension(&self.circuit_file, "r1cs")
            .display()
            .to_string();

        let mut create_z_key_cmd = SnarkjsCmd {
            pot_dir: &self.circuit_dir,
            args: &[
                "groth16",
                "setup",
                &r1cs_file,
                &contribution_file_final,
                &self.zkey_file_0().display().to_string(),
            ],
            description: "Create .zkey",
            output,
        };

        create_z_key_cmd.run()
    }

    pub fn contribute_z_key(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Contribute .zkey ...")?;

        let mut contribute_z_key_cmd = SnarkjsCmd {
            pot_dir: &self.circuit_dir,
            args: &[
                "zkc",
                &self.zkey_file_0().as_path().display().to_string(),
                &self.zkey_file_1().as_path().display().to_string(),
                "-v",
                "-e",
            ],
            description: "Create .zkey",
            output,
        };

        let contribute_z_key_cmd_success = contribute_z_key_cmd.run()?;

        if !contribute_z_key_cmd_success {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn export_verification_key(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Export verification key...")?;

        let mut contribute_z_key_cmd = SnarkjsCmd {
            pot_dir: &self.circuit_dir,
            args: &[
                "zkev",
                &self.zkey_file_1().as_path().display().to_string(),
                &self
                    .verification_file_name()
                    .as_path()
                    .display()
                    .to_string(),
            ],
            description: "Export json verification key",
            output,
        };

        contribute_z_key_cmd.run()
    }

    pub fn generate_proof(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Generating proof...")?;

        let witness_folder_with_name = format!("{}/{}", &self.compiled_folder(), "witness.wtns");

        let mut generate_proof_cmd = SnarkjsCmd {
            pot_dir: &self.circuit_dir,
            args: &[
                "g16p",
                &self.zkey_file_1().as_path().display().to_string(),
                &witness_folder_with_name,
                &self.proof_file_name().as_path().display().to_string(),
                &self.public_key_file_name().as_path().display().to_string(),
            ],
            description: "Generate proof file",
            output,
        };

        generate_proof_cmd.run()
    }

    pub fn verify_proof(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Verifying proof...")?;

        let mut verify_proof_cmd = SnarkjsCmd {
            pot_dir: &self.circuit_dir,
            args: &[
                "g16v",
                &self.verification_file_name().display().to_string(),
                &self.public_key_file_name().display().to_string(),
                &self.proof_file_name().display().to_string(),
            ],
            description: "Verify proof",
            output,
        };

        verify_proof_cmd.run()
    }

    fn compiled_folder(&self) -> String {
        return self
            .circuit_file
            .to_str()
            .unwrap()
            .replace(".circom", "_js");
    }

    fn input_file_dir(&self) -> String {
        let input_file = change_extension(&self.circuit_file, "json")
            .display()
            .to_string();
        format!("{:?}/{}", "../", input_file).replace('"', "")
    }

    // Ceremony setup
    // fn contribution_file_0(&self) -> String {
    //     self.contribution_file_name("_0000")
    // }

    // fn contribution_file_1(&self) -> String {
    //     self.contribution_file_name("_0001")
    // }

    // fn contribution_file_final(&self) -> String {
    //     self.contribution_file_name("_final")
    // }

    // fn contribution_file_name(&self, suffix: &str) -> String {
    //     format!("pot{}{}.ptau", &self.ptau, &suffix)
    // }

    // zkey
    fn zkey_file_0(&self) -> PathBuf {
        change_extension_with_suffix(&self.circuit_file, "_0000", "zkey")
    }

    fn zkey_file_1(&self) -> PathBuf {
        change_extension_with_suffix(&self.circuit_file, "_0001", "zkey")
    }

    fn verification_file_name(&self) -> PathBuf {
        change_extension_with_suffix(&self.circuit_file, "_verification", "json")
    }

    fn public_key_file_name(&self) -> PathBuf {
        change_extension_with_suffix(&self.circuit_file, "_out", "json")
    }

    fn proof_file_name(&self) -> PathBuf {
        change_extension_with_suffix(&self.circuit_file, "_prove", "json")
    }
}
