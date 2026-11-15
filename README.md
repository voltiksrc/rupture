# Rupture

Rupture is a small, self-modifying esoteric programming language written in Rust.

It has 64 KiB of shared memory for both code and data, so programs can modify their own instructions while running.

It's usable, but pretty painful to actually program in.

## Instructions

| Instruction | Description |
|---|---|
| `%++` | Increment the current memory cell (wraps at 255) |
| `%--` | Decrement the current memory cell (wraps at 0) |
| `>>` | Move the memory pointer right |
| `<<` | Move the memory pointer left |
| `^777<text>^` | Print text (supports `\n`) |
| `#999` | Print the current cell's value as a number |
| `?IN` | Read one byte from standard input |
| `cc(a,b)` | Copy a byte from address `a` to address `b` |
| `:{n}` | Jump to byte address `n` |
| `?0-:{n}` | Jump to byte address `n` if the current cell is zero |
| `[` | Start a loop that runs while the current cell is nonzero |
| `]` | End a loop |
| `^^text^^` | Comment |
| `41C5` | Halt execution |

Whitespace is ignored between instructions, so programs can be written across multiple lines.

## Usage

Build with Cargo:

```sh
cargo build --release
```

Run a `.rp` program:

```sh
cargo run -- examples/test.rp
```

Or run the compiled binary directly:

```sh
./target/release/rupture examples/test.rp
```

### Example

```text
^^ Hello, world! ^^

^777<Hello, world!\n>^
41C5
```

Another example, using memory operations:

```text
^^ Increment the current cell three times and print its value ^^

%++
%++
%++

#999
41C5
```

This prints `3`.

## Syntax highlighting

Rupture has its own Tree-sitter grammar, which I made for syntax highlighting in Mitos.

You can find it here: [tree-sitter-rupture](https://github.com/voltiksrc/tree-sitter-rupture).

## Notes

- Memory is limited to 65,536 bytes.
- Code and data share the same memory.
- The memory pointer starts immediately after the loaded source code.
- Jump addresses are absolute byte offsets, not line numbers.
- Input reads a single byte, not an entire line.
- Invalid instructions and out-of-bounds memory accesses result in errors.
- Since code is stored in writable memory, programs can overwrite their own instructions.

## License

Rupture is licensed under the GNU General Public License v3.0 or later. See [LICENSE](LICENSE) for details.
