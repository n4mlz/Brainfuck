# Rust Brainfuck Interpreter

A minimal, clean, and modular Brainfuck interpreter written in Rust.

## Overview

Cleanly separated into three main components:

- `lexer` — Tokenizes Brainfuck source code into `Token` enums.
- `parser` — Parses tokens into an AST represented by `Node` enums, supporting nested loops.
- `interpreter` — Executes the AST on a memory tape with I/O abstraction.

## Usage

1. **Clone** the repository:

```bash
git clone https://github.com/n4mlz/Brainfuck.git
cd Brainfuck
```

2. **Run** the sample program or your own `.bf` file:

```bash
cargo run sample.bf
```

## Sample

```bf
>+[+[<]>>+<+]>.
```

This program outputs the character `A`.

## License

This project is licensed under the MIT License. Feel free to use and modify it for your own projects.
