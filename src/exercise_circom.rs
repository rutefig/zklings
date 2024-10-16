use anyhow::{Ok, Result};

use std::path::Path;
use std::{ffi::OsStr, io::Write};

use crossterm::style::Stylize;

use crate::path::change_extension_with_suffix;
use crate::{
    cmd::WasmWitnessCmd,
    cmd_snarkjs::SnarkjsCmd,
    path::{append_compiled_folder, change_extension},
};

pub fn generate_witness(
    output: &mut Vec<u8>,
    circuit_dir: &Path,
    circuit_file: &OsStr,
) -> Result<bool> {
    writeln!(output, "{}", "Computing witness...".underlined())?;

    let input_file = change_extension(circuit_file, "json").display().to_string();
    let input_file_dir = format!("{:?}/{}", "../", input_file).replace('"', "");
    let compiled_folder: String = circuit_file.to_str().unwrap().replace(".circom", "_js");
    let compiled_dir = append_compiled_folder(circuit_dir, &compiled_folder);
    let wasm_file = change_extension(circuit_file, "wasm").display().to_string();

    let mut generate_witness_cmd = WasmWitnessCmd {
        args: &[&wasm_file, &input_file_dir, "witness.wtns"],
        description: "Computing witness",
        output,
        compiled_dir: &compiled_dir,
    };

    let generate_witness_success = generate_witness_cmd.run()?;

    if !generate_witness_success {
        return Ok(false);
    }

    Ok(generate_witness_success)
}

// "powersoftau new bn128 9 pot9_0000.ptau -v"
pub fn start_ceremony(output: &mut Vec<u8>, pot_dir: &Path) -> Result<bool> {
    writeln!(output, "{}", "Start ceremony...")?;

    let mut start_ceremony_cmd = SnarkjsCmd {
        pot_dir,
        args: &["powersoftau", "new", "bn128", "9", "pot9_0000.ptau", "-v"],
        description: "Start power of tau ceremony",
        output,
    };

    let start_ceremony_success = start_ceremony_cmd.run()?;

    if !start_ceremony_success {
        return Ok(false);
    }

    Ok(start_ceremony_success)
}

pub fn contribute_ceremony(
    output: &mut Vec<u8>,
    pot_dir: &Path,
    contribute_in_file_name: &Path,
    contribute_out_file_name: &Path,
) -> Result<bool> {
    writeln!(output, "{}", "Contribute to the ceremony...")?;

    // Create ceremony with skipping entropy input
    let mut contribute_ceremony_cmd = SnarkjsCmd {
        pot_dir,
        args: &[
            "ptc",
            &contribute_in_file_name.display().to_string(),
            &contribute_out_file_name.display().to_string(),
            "-v",
            "-e",
        ],
        description: "First contribution",
        output,
    };

    let contribute_ceremony_success = contribute_ceremony_cmd.run()?;

    if !contribute_ceremony_success {
        return Ok(false);
    }

    Ok(true)
}

pub fn prepare_circuit_proof(output: &mut Vec<u8>, pot_dir: &Path) -> Result<bool> {
    writeln!(output, "{}", "Prepare circuit...")?;

    let mut prepare_circuit_proof_cmd = SnarkjsCmd {
        pot_dir,
        args: &["pt2", "pot9_0001.ptau", "pot9_final.ptau", "-v", "-e"],
        description: "Prepare circuit-specific",
        output,
    };

    let prepare_circuit_proof_cmd_success = prepare_circuit_proof_cmd.run()?;

    if !prepare_circuit_proof_cmd_success {
        return Ok(false);
    }

    Ok(true)
}

pub fn create_z_key(output: &mut Vec<u8>, pot_dir: &Path, circuit_file: &OsStr) -> Result<bool> {
    writeln!(output, "{}", "Create .zkey ...")?;

    let r1cs_file = change_extension(circuit_file, "r1cs").display().to_string();
    let z_key_file_name: String = change_extension_with_suffix(circuit_file, "_0000", "zkey")
        .display()
        .to_string();

    let mut create_z_key_cmd = SnarkjsCmd {
        pot_dir,
        args: &[
            "groth16",
            "setup",
            &r1cs_file,
            "pot9_final.ptau",
            &z_key_file_name,
        ],
        description: "Create .zkey",
        output,
    };

    let create_z_key_cmd_success = create_z_key_cmd.run()?;

    if !create_z_key_cmd_success {
        return Ok(false);
    }

    Ok(true)
}

pub fn contribute_z_key(
    output: &mut Vec<u8>,
    pot_dir: &Path,
    circuit_file: &OsStr,
) -> Result<bool> {
    writeln!(output, "{}", "Contribute .zkey ...")?;

    let z_key_in_file_name = change_extension_with_suffix(circuit_file, "_0000", "zkey");
    let z_key_out_file_name = change_extension_with_suffix(circuit_file, "_0001", "zkey");

    let mut contribute_z_key_cmd = SnarkjsCmd {
        pot_dir,
        args: &[
            "zkc",
            &z_key_in_file_name.as_path().display().to_string(),
            &z_key_out_file_name.as_path().display().to_string(),
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

pub fn export_verification_key(
    output: &mut Vec<u8>,
    pot_dir: &Path,
    circuit_file: &OsStr,
) -> Result<bool> {
    writeln!(output, "{}", "Export verification key...")?;

    let z_key_in_file_name = change_extension_with_suffix(circuit_file, "_0001", "zkey");
    let json_key_file_name = change_extension_with_suffix(circuit_file, "_verification", "json");

    let mut contribute_z_key_cmd = SnarkjsCmd {
        pot_dir,
        args: &[
            "zkev",
            &z_key_in_file_name.as_path().display().to_string(),
            &json_key_file_name.as_path().display().to_string(),
        ],
        description: "Export json verification key",
        output,
    };

    let contribute_z_key_cmd_success = contribute_z_key_cmd.run()?;

    if !contribute_z_key_cmd_success {
        return Ok(false);
    }

    Ok(true)
}

pub fn generate_proof(output: &mut Vec<u8>, pot_dir: &Path, circuit_file: &OsStr) -> Result<bool> {
    writeln!(output, "{}", "Generating proof...")?;

    let compiled_folder = circuit_file.to_str().unwrap().replace(".circom", "_js");
    let witness_folder_with_name = format!("{}/{}", compiled_folder, "witness.wtns");

    let z_key_in_file_name = change_extension_with_suffix(circuit_file, "_0001", "zkey");
    let proof_file_name = change_extension_with_suffix(circuit_file, "_prove", "json");
    let json_key_file_name = change_extension_with_suffix(circuit_file, "_out", "json");

    let mut generate_proof_cmd = SnarkjsCmd {
        pot_dir: &pot_dir,
        args: &[
            "g16p",
            &z_key_in_file_name.as_path().display().to_string(),
            &witness_folder_with_name,
            &proof_file_name.as_path().display().to_string(),
            &json_key_file_name.as_path().display().to_string(),
        ],
        description: "Generate proof file",
        output,
    };

    let generate_proof_cmd_success = generate_proof_cmd.run()?;

    if !generate_proof_cmd_success {
        return Ok(false);
    }

    Ok(true)
}

pub fn verify_proof(output: &mut Vec<u8>, pot_dir: &Path, circuit_file: &OsStr) -> Result<bool> {
    writeln!(output, "{}", "Verifying proof...")?;

    let verification_key_file_name =
        change_extension_with_suffix(circuit_file, "_verification", "json");
    let public_key_file_name = change_extension_with_suffix(circuit_file, "_out", "json");
    let proof_file_name = change_extension_with_suffix(circuit_file, "_prove", "json");

    let mut verify_proof_cmd = SnarkjsCmd {
        pot_dir: &pot_dir,
        args: &[
            "g16v",
            &verification_key_file_name.display().to_string(),
            &public_key_file_name.display().to_string(),
            &proof_file_name.display().to_string(),
        ],
        description: "Verify proof",
        output,
    };

    let verify_proof_cmd_success = verify_proof_cmd.run()?;

    if !verify_proof_cmd_success {
        return Ok(false);
    }

    Ok(true)
}
