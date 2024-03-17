//!
//! # CLI argument handler
//!
//! This module handles the CLI arguments using clap

use clap::{crate_version, value_parser, Arg, ArgAction, Command};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn build_cli() -> Command {
    Command::new("perfer")
        .name("perfer")
        .author("Væñgír, <vaengir@gmx.de>")
        .version(crate_version!())
        .about("A CLI tool which let's you track the memory used by a program.")
        .arg(
            Arg::new("process")
                .required(true)
                .help("Command you wish to track\nMulti word commands need to be put in quotes.")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Use verbose output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

pub(crate) fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    println!("-----------------------------------------------------------------------------------------------------");
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    println!("-----------------------------------------------------------------------------------------------------");
    println!("Copy everything between the lines into the corresponding dir for the shell you use.");
}
