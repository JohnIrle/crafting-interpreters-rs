// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT
//
// This file is part of rlox-ast

fn main() {
    let mut runner = rlox::Runner::new();
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => runner.run_prompt(),
        2 => runner.run_file(&args[1]),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
    println!("Hello, world!");
}
