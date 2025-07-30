use std::io::{self, Read, Write};

use crate::parser::Node;

const MEMORY_SIZE: usize = 30000;

pub struct Interpreter<R: Read, W: Write> {
    memory: Vec<u8>,
    ptr: usize,
    input: R,
    output: W,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(input: R, output: W) -> Self {
        Interpreter {
            memory: vec![0; MEMORY_SIZE],
            ptr: 0,
            input,
            output,
        }
    }

    pub fn run(&mut self, nodes: &[Node]) -> io::Result<()> {
        for node in nodes {
            match node {
                Node::IncPtr => self.ptr = self.ptr.wrapping_add(1),
                Node::DecPtr => self.ptr = self.ptr.wrapping_sub(1),
                Node::IncCell => self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(1),
                Node::DecCell => self.memory[self.ptr] = self.memory[self.ptr].wrapping_sub(1),
                Node::Output => {
                    self.output.write_all(&[self.memory[self.ptr]])?;
                }
                Node::Input => {
                    let mut buf = [0];
                    if self.input.read_exact(&mut buf).is_ok() {
                        self.memory[self.ptr] = buf[0];
                    }
                }
                Node::Loop(body) => {
                    while self.memory[self.ptr] != 0 {
                        self.run(body)?;
                    }
                }
            }
        }
        Ok(())
    }
}
