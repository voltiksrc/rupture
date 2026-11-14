# Rupture

Rupture is a small, self-modifying esoteric programming language written in Rust. Code and data share the same 64 KiB of memory, meaning programs can overwrite their own instructions while running.

It's designed to be usable, but unnecessarily painful to program in.

## Instructions

| Instruction | Description |
|---|---|
| `%++` | Increment the current memory cell |
| `%--` | Decrement the current memory cell |
| `>>` | Move the memory pointer right |
| `<<` | Move the memory pointer left |
| `^777<text>^` | Print text (supports `\n`) |
| `cc(a,b)` | Copy a byte from address `a` to `b` |
| `:{n}` | Jump to byte address `n` |
| `41C5` | Halt execution |

Conditional jumps (`?0-:{n}`) are planned.

## Usage

Build with Cargo:

```sh
cargo build --release
```

Run a `.rp` program:

```sh
cargo run -- examples/test.rp
```

Example:

```text
^777<Hello, world!\n>^41C5
```

## Notes

- Memory is limited to 65,536 bytes.
- Code and data share the same memory.
- Jump addresses are byte offsets, not line numbers.
- Invalid instructions result in an error.
- There are no comments.

## License

Rupture is licensed under the GNU General Public License v3.0 or later.
See [LICENSE](LICENSE) for details.
