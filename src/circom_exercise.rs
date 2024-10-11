use anyhow::Result;

use std::path::Path;
use std::{ffi::OsStr, io::Write};

use crossterm::style::Stylize;

use crate::{
    cmd::WasmWitnessCmd,
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
    let compiled_folder = circuit_file.to_str().unwrap().replace(".circom", "_js");
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
