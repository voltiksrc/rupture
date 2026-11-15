// SPDX-License-Identifier: GPL-3.0

mod decode;
#[cfg(test)]
mod tests;
mod vm;

use std::{env, fs};
use vm::Vm;

fn main() -> Result<(), String> {
    let path = env::args().nth(1).ok_or("Usage rupture <file.rp>")?;

    let source = fs::read(&path).map_err(|err| format!("Failed to read '{}': {}", path, err))?;

    let mut vm = Vm::new(&source)?;

    vm.run()?;

    Ok(())
}
