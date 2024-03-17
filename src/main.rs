//!
//! # perfer
//!
//! A CLI tool which let's you track the memory used by a program.

use crate::commands::cli::{self, print_completions};
use anyhow::{Context, Result};
use clap_complete::Shell;
use commands::cli::build_cli;
use std::process::{Command, Stdio};
use wait4::Wait4;

pub(crate) mod commands;

fn main() -> Result<()> {
    let args = cli::build_cli().get_matches();

    if let Some(generator) = args.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }

    if let Some(process) = args.get_many::<String>("process") {
        let verbose = args.get_flag("verbose");
        let mut command: Vec<String> = process.into_iter().cloned().collect();
        if command.len() == 1 && command[0].contains(' ') {
            command = command[0]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }
        let mut cmd = Command::new(command[0].clone());
        for arg in command.iter().skip(1) {
            cmd.arg(arg);
        }
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Couldn't get output")?;
        let mut process = cmd.spawn().context("Failed spawning the given process")?;
        let resources = process
            .wait4()
            .context("Couldn't wait for process to finish")?;
        if resources.status.success() {
            if verbose {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            println!("------------");
            println!("Process completed successfully:");
            println!("  User time: {}ms", resources.rusage.utime.as_millis());
            println!("  Kernel time: {}ms", resources.rusage.stime.as_millis());
            println!("  Memory use: {}KB", resources.rusage.maxrss / 1024);
        } else {
            println!("Process errored with {}\n", resources.status);
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(())
}
