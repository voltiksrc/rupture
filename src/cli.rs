// SPDX-License-Identifier: GPL-3.0

use std::env;

pub enum Command {
    Help,
    Version,
    Run(String),
}

pub fn parse() -> Result<Command, String> {
    match env::args().nth(1).as_deref() {
        Some("-h" | "--help") => Ok(Command::Help),
        Some("-V" | "--version") => Ok(Command::Version),
        Some(arg) if arg.starts_with('-') => Err(format!("Unknown option: {arg}")),
        Some(path) => Ok(Command::Run(path.to_string())),
        None => Err("Missing source file. Try 'rupture --help'.".into()),
    }
}

pub fn help() {
    println!(
        "\
Usage: rupture [OPTION]... FILE

Execute a Rupture program.

Options:
  -h, --help       Display this help and exit
  -V, --version    Output version information and exit

Report bugs to: https://github.com/voltiksrc/rupture/issues"
    );
}

pub fn version() {
    println!(
        "\
rupture {}
Copyright (C) 2026 voltik
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by voltik.",
        env!("CARGO_PKG_VERSION")
    );
}
