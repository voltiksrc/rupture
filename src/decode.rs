// SPDX-License-Identifier: GPL-3.0

pub enum Instruction {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    LoopStart,
    LoopEnd,
    Halt,
    Copy(usize, usize),
    Jump(usize),
    JumpIfZero(usize),
    Print(String),
}

pub fn decode(memory: &[u8], pc: usize) -> Result<(Instruction, usize), String> {
    if memory.get(pc..pc + 3) == Some(b"%++".as_slice()) {
        return Ok((Instruction::Increment, 3));
    }

    if memory.get(pc..pc + 3) == Some(b"%--".as_slice()) {
        return Ok((Instruction::Decrement, 3));
    }

    if memory.get(pc..pc + 2) == Some(b">>".as_slice()) {
        return Ok((Instruction::MoveRight, 2));
    }

    if memory.get(pc..pc + 2) == Some(b"<<".as_slice()) {
        return Ok((Instruction::MoveLeft, 2));
    }

    if memory.get(pc..pc + 4) == Some(b"41C5".as_slice()) {
        return Ok((Instruction::Halt, 4));
    }

    if memory.get(pc) == Some(&b'[') {
        return Ok((Instruction::LoopStart, 1));
    }

    if memory.get(pc) == Some(&b']') {
        return Ok((Instruction::LoopEnd, 1));
    }

    if memory.get(pc..pc + 5) == Some(b"^777<".as_slice()) {
        let remaining = &memory[pc + 5..];

        let end = remaining
            .windows(2)
            .position(|pair| pair == b">^")
            .ok_or("Unterminated print instruction")?;

        let text = String::from_utf8(remaining[..end].to_vec())
            .map_err(|_| "Invalid UTF-8 in print instruction")?
            .replace("\\n", "\n");

        return Ok((Instruction::Print(text), 5 + end + 2));
    }

    if memory.get(pc..pc + 3) == Some(b"cc(".as_slice()) {
        let remaining = &memory[pc + 3..];

        let end = remaining
            .iter()
            .position(|&byte| byte == b')')
            .ok_or("Unfinished copy instruction")?;

        let args = std::str::from_utf8(&remaining[..end])
            .map_err(|_| "Invalid UTF-8 in copy instruction")?;

        let (src, dst) = args
            .split_once(',')
            .ok_or("The copy instruction requires two addresses")?;

        let source = src.parse::<usize>().map_err(|_| "Invalid source address")?;

        let destination = dst
            .parse::<usize>()
            .map_err(|_| "Invalid destination address")?;

        return Ok((Instruction::Copy(source, destination), 3 + end + 1));
    }

    if memory.get(pc..pc + 2) == Some(b":{".as_slice()) {
        let remaining = &memory[pc + 2..];

        let end = remaining
            .iter()
            .position(|&byte| byte == b'}')
            .ok_or("Unterminated jump instruction")?;

        let args = std::str::from_utf8(&remaining[..end])
            .map_err(|_| "Invalid UTF-8 in jump instruction")?;

        let address = args.parse::<usize>().map_err(|_| "Invalid address")?;

        return Ok((Instruction::Jump(address), 2 + end + 1));
    }

    if memory.get(pc..pc + 5) == Some(b"?0-:{".as_slice()) {
        let remaining = &memory[pc + 5..];

        let end = remaining
            .iter()
            .position(|&byte| byte == b'}')
            .ok_or("Unterminated jump if zero instruction")?;

        let args = std::str::from_utf8(&remaining[..end])
            .map_err(|_| "Invalid UTF-8 in jump instruction")?;

        let address = args.parse::<usize>().map_err(|_| "Invalid address")?;

        return Ok((Instruction::JumpIfZero(address), 5 + end + 1));
    }

    Err(format!("Unknown instruction at address {}", pc))
}
