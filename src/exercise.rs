use anyhow::Result;
use crossterm::style::{style, StyledContent, Stylize};
use markdown::{mdast::Node, to_mdast, ParseOptions};
use std::{
    fmt::{self, Display, Formatter},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

use crate::{
    cmd::{run_cmd, CargoCmd, CircomCmd},
    exercise_circom::CircomExercise,
    in_official_repo,
    terminal_link::TerminalFileLink,
    time::get_elapsed_time,
    DEBUG_PROFILE,
};

/// The initial capacity of the output buffer.
pub const OUTPUT_CAPACITY: usize = 1 << 14;

// Run an exercise binary and append its output to the `output` buffer.
// Compilation must be done before calling this method.
fn run_bin(bin_name: &str, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
    writeln!(output, "{}", "Output".underlined())?;

    // 7 = "/debug/".len()
    let mut bin_path = PathBuf::with_capacity(target_dir.as_os_str().len() + 7 + bin_name.len());
    bin_path.push(target_dir);
    bin_path.push("debug");
    bin_path.push(bin_name);

    let success = run_cmd(Command::new(&bin_path), &bin_path.to_string_lossy(), output)?;

    if !success {
        // This output is important to show the user that something went wrong.
        // Otherwise, calling something like `exit(1)` in an exercise without further output
        // leaves the user confused about why the exercise isn't done yet.
        writeln!(
            output,
            "{}",
            "The exercise didn't run successfully (nonzero exit code)"
                .bold()
                .red(),
        )?;
    }

    Ok(success)
}

/// See `info_file::ExerciseInfo`
pub struct Exercise {
    pub dir: Option<&'static str>,
    pub name: &'static str,
    pub ext: &'static str,
    /// Path of the exercise file starting with the `exercises/` directory.
    pub path: &'static str,
    pub test: bool,
    pub strict_clippy: bool,
    pub hint: String,
    pub done: bool,
}

impl Exercise {
    pub fn terminal_link(&self) -> StyledContent<TerminalFileLink<'_>> {
        style(TerminalFileLink(self.path)).underlined().blue()
    }

    pub fn readme_link(&self) -> StyledContent<TerminalFileLink<'static>> {
        let path = self
            .path
            .replace(self.ext, "")
            .replace(self.name, "README.md");
        let boxed_path = Box::leak(path.into_boxed_str());
        style(TerminalFileLink(boxed_path)).underlined().blue()
    }

    pub fn is_rust(&self) -> bool {
        self.ext == "rs"
    }

    pub fn is_circom(&self) -> bool {
        self.ext == "circom"
    }

    pub fn is_md(&self) -> bool {
        self.ext == "md"
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.path.fmt(f)
    }
}

pub trait RunnableExercise {
    fn name(&self) -> &str;
    fn strict_clippy(&self) -> bool;
    fn test(&self) -> bool;
    fn is_rust(&self) -> bool;
    fn is_circom(&self) -> bool;
    fn is_md(&self) -> bool;
    fn path(&self) -> String;

    // Compile, check and run the exercise or its solution (depending on `bin_name´).
    // The output is written to the `output` buffer after clearing it.
    fn run(&self, bin_name: &str, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        output.clear();

        // Developing the official Rustlings.
        let dev = DEBUG_PROFILE && in_official_repo();

        let build_success = CargoCmd {
            subcommand: "build",
            args: &[],
            bin_name,
            description: "cargo build …",
            hide_warnings: false,
            target_dir,
            output,
            dev,
        }
        .run()?;
        if !build_success {
            return Ok(false);
        }

        // Discard the output of `cargo build` because it will be shown again by Clippy.
        output.clear();

        // `--profile test` is required to also check code with `[cfg(test)]`.
        let clippy_args: &[&str] = if self.strict_clippy() {
            &["--profile", "test", "--", "-D", "warnings"]
        } else {
            &["--profile", "test"]
        };
        let clippy_success = CargoCmd {
            subcommand: "clippy",
            args: clippy_args,
            bin_name,
            description: "cargo clippy …",
            hide_warnings: false,
            target_dir,
            output,
            dev,
        }
        .run()?;
        if !clippy_success {
            return Ok(false);
        }

        if !self.test() {
            return run_bin(bin_name, output, target_dir);
        }

        let test_success = CargoCmd {
            subcommand: "test",
            args: &["--", "--color", "always", "--show-output"],
            bin_name,
            description: "cargo test …",
            // Hide warnings because they are shown by Clippy.
            hide_warnings: true,
            target_dir,
            output,
            dev,
        }
        .run()?;

        let run_success = run_bin(bin_name, output, target_dir)?;

        Ok(test_success && run_success)
    }

    /// Function for running Circom exercises
    fn run_circom(&self, output: &mut Vec<u8>, _target_dir: &Path) -> Result<bool> {
        writeln!(output, "{}", "Compiling Circom circuit...".underlined())?;

        // For logging elapsed time for whole process.
        let now = SystemTime::now();

        let path = self.path().clone();
        let full_path = Path::new(&path);
        let circuit_dir = full_path.parent().unwrap_or(Path::new(""));
        let circuit_file = full_path.file_name().unwrap_or_default();

        writeln!(output, "Circuit directory: {:?}", circuit_dir)?;
        writeln!(output, "Circuit file: {:?}", circuit_file)?;

        let mut compile_cmd = CircomCmd {
            args: &["--r1cs", "--wasm", "--sym"],
            circuit_name: circuit_file.to_str().unwrap_or(self.name()),
            description: "Compiling Circom circuit",
            output,
            circuit_dir,
        };

        let compile_success = compile_cmd.run()?;

        if !compile_success {
            return Ok(false);
        }

        // Circom.
        let ptau = "9";
        let circom_exercise = CircomExercise {
            circuit_dir,
            circuit_file,
            ptau,
        };

        // Computing witness.
        let generate_witness = circom_exercise.generate_witness(output)?;
        if !generate_witness {
            return Ok(false);
        }

        // Setup ceremony and generate proof
        writeln!(output, "{}", "Generating proof...".underlined())?;

        circom_exercise.start_ceremony(output)?;
        circom_exercise.contribute_ceremony(output)?;
        // Note: Circuit specific quite taking time,
        // maybe having flag to check if exercise need to check is nice to have.
        circom_exercise.prepare_circuit_proof(output)?;
        circom_exercise.create_z_key(output)?;
        circom_exercise.contribute_z_key(output)?;
        circom_exercise.export_verification_key(output)?;
        let proof_success = circom_exercise.generate_proof(output).unwrap();
        if !proof_success {
            return Ok(false);
        }

        writeln!(output, "{}", "Verifying proof...".underlined())?;
        let verify_success = circom_exercise.verify_proof(output).unwrap();
        if !verify_success {
            return Ok(false);
        }

        let elapsed_time = get_elapsed_time(&now);
        writeln!(output, "Elapsed time: {}s", elapsed_time)?;

        Ok(true)
    }

    fn run_markdown(&self, output: &mut Vec<u8>) -> Result<bool> {
        let user_content = fs::read_to_string(self.path())?;
        let solution_content = fs::read_to_string(self.sol_path())?;

        println!("User content:\n{}", user_content);
        println!("Solution content:\n{}", solution_content);

        let user_ast = to_mdast(&user_content, &ParseOptions::gfm()).unwrap();
        let solution_ast = to_mdast(&solution_content, &ParseOptions::gfm()).unwrap();

        let (user_question, user_answer) = self.extract_question_and_answer(&user_ast)?;
        let (solution_question, solution_answer) =
            self.extract_question_and_answer(&solution_ast)?;

        println!(
            "User question: '{}'\nUser answer: '{}'",
            user_question.trim(),
            user_answer.trim()
        );
        println!(
            "Solution question: '{}'\nSolution answer: '{}'",
            solution_question.trim(),
            solution_answer.trim()
        );

        let success = user_answer.trim() == solution_answer.trim();
        if success {
            writeln!(
                output,
                "Correct! Your solution matches the expected answer."
            )?;
            writeln!(output, "Your answer: '{}'", user_answer.trim())?;
        } else {
            writeln!(output, "{}", "Fix me!".red())?;
            writeln!(output, "Your answer doesn't match the expected solution.")?;
            writeln!(output, "Your answer: '{}'", user_answer.trim())?;
            writeln!(
                output,
                "Check the file below and write the correct solution to the proposed problem."
            )?;
        }

        Ok(success)
    }

    fn sol_path(&self) -> String;

    fn extract_question_and_answer(&self, ast: &Node) -> Result<(String, String)> {
        let mut question = String::new();
        let mut answer = String::new();
        let mut in_question = false;

        if let Node::Root(root) = ast {
            for child in &root.children {
                match child {
                    Node::Heading(heading) if heading.depth == 1 => {
                        in_question = true;
                        for child in &heading.children {
                            if let Node::Text(text) = child {
                                question.push_str(&text.value);
                            }
                        }
                    }
                    Node::Paragraph(para) if in_question => {
                        for child in &para.children {
                            if let Node::Text(text) = child {
                                question.push_str(&text.value);
                            }
                        }
                    }
                    Node::Code(code) if code.lang.as_deref() == Some("math") => {
                        answer = code.value.trim().to_string();
                        break;
                    }
                    _ => {}
                }
            }
        }

        if question.is_empty() || answer.is_empty() {
            anyhow::bail!("Failed to extract question or answer from markdown");
        }

        println!("Extracted question: '{}'", question.trim());
        println!("Extracted answer: '{}'", answer.trim());

        Ok((question, answer))
    }

    /// Compile, check and run the exercise.
    /// The output is written to the `output` buffer after clearing it.
    #[inline]
    fn run_exercise(&self, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        if self.is_rust() {
            self.run(self.name(), output, target_dir)
        } else if self.is_circom() {
            self.run_circom(output, target_dir)
        } else if self.is_md() {
            self.run_markdown(output)
        } else {
            anyhow::bail!("Unsupported exercise type")
        }
    }

    /// Compile, check and run the exercise's solution.
    /// The output is written to the `output` buffer after clearing it.
    fn run_solution(&self, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        let name = self.name();
        let mut bin_name = String::with_capacity(name.len());
        bin_name.push_str(name);
        bin_name.push_str("_sol");

        self.run(&bin_name, output, target_dir)
    }
}

impl RunnableExercise for Exercise {
    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    #[inline]
    fn path(&self) -> String {
        self.path.to_string()
    }

    #[inline]
    fn strict_clippy(&self) -> bool {
        self.strict_clippy
    }

    #[inline]
    fn test(&self) -> bool {
        self.test
    }

    #[inline]
    fn is_rust(&self) -> bool {
        self.is_rust()
    }

    #[inline]
    fn is_circom(&self) -> bool {
        self.is_circom()
    }

    #[inline]
    fn is_md(&self) -> bool {
        self.is_md()
    }

    #[inline]
    fn sol_path(&self) -> String {
        let exercise_path = self.path();
        println!("PATH: {}", self.path);

        exercise_path.replace("exercises", "solutions")
    }
}
