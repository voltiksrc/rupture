// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

mod cli;
mod decode;
mod instructions;
mod vm;

use std::fs;
use vm::Vm;

fn main() -> Result<(), String> {
    match cli::parse()? {
        cli::Command::Help => cli::help(),
        cli::Command::Version => cli::version(),
        cli::Command::Run(path) => {
            let source =
                fs::read(&path).map_err(|err| format!("Failed to read '{}': {}", path, err))?;

            let mut vm = Vm::new(&source)?;
            vm.run()?;
        }
    }

    Ok(())
}
