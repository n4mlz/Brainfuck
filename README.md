# Rust Brainfuck Interpreter & Compiler

A minimal, clean, and modular Brainfuck interpreter and compiler written in Rust.

## Overview

Cleanly separated into four main components:

- `lexer` — Tokenizes Brainfuck source code into `Token` enums.
- `parser` — Parses tokens into an AST represented by `Node` enums, supporting nested loops.
- `interpreter` — Executes the AST on a memory tape with I/O abstraction.
- `codegen` — Generates LLVM IR code from the AST for compilation.

## Usage

1. **Clone** the repository:

```bash
git clone https://github.com/n4mlz/Brainfuck.git
cd Brainfuck
```

2. **Run** the sample program or your own `.bf` file:

### Interpret mode:

```bash
cargo run interpret sample.bf
# or
cargo run i sample.bf
```

### Compile mode (outputs LLVM IR):

```bash
cargo run compile sample.bf
# or
cargo run c sample.bf
```

To compile and execute the program:

```bash
cargo run c sample.bf > out.ll
lli out.ll
```

## Sample

```bf
>+[+[<]>>+<+]>.
```

This program outputs the character `A`.

## License

This project is licensed under the MIT License. Feel free to use and modify it for your own projects.
