// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT

use rlox::chunk::Chunk;
use rlox::chunk::OpCode::{OP_CONSTANT, OP_RETURN};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OP_CONSTANT.value(), 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OP_RETURN.value(), 123);
    chunk.disassemble_chunk("test chunk");
}
