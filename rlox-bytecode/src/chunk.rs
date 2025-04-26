// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT

use crate::grow_capacity;
use crate::value::{Value, ValueArray};

#[allow(non_camel_case_types)]
pub enum OpCode {
    OP_CONSTANT,
    OP_RETURN,
}

impl OpCode {
    pub fn value(&self) -> u8 {
        match self {
            Self::OP_CONSTANT => 0,
            Self::OP_RETURN => 1,
        }
    }
}

#[derive(Default)]
pub struct Chunk {
    count: i32,
    capacity: i32,
    code: Vec<u8>,
    lines: Vec<i32>,
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> i32 {
        self.constants.write_value(value);
        self.constants.count - 1
    }

    pub fn write_chunk(&mut self, byte: u8, line: i32) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
        }

        self.code.insert(self.count as usize, byte);
        self.lines.insert(self.count as usize, line);
        self.count += 1;
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.count {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: i32) -> i32 {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset as usize] == self.lines[offset as usize - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset as usize]);
        }

        let instruction = self.code[offset as usize];
        match instruction {
            0 => self.constant_instruction("OP_CONSTANT", offset),
            1 => self.simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {}", instruction);

                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: i32) -> i32 {
        println!("{}", name);

        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: i32) -> i32 {
        let constant = self.code[offset as usize + 1];
        print!("{:<16} {:4} '", name, constant);
        print!("{}", self.constants.values[constant as usize]);
        println!("'");

        offset + 2
    }
}
