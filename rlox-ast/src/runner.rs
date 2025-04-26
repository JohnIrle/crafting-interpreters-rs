// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT

use crate::ast_printer::AstPrinter;
use std::io::Write;

pub struct Runner {
    had_error: bool,
}

impl Runner {
    pub fn new() -> Self {
        Self { had_error: false }
    }
    pub fn run_file(&mut self, path: &String) {
        let source = std::fs::read_to_string(path).expect("Failed to read file");
        self.run(source);

        if self.had_error {
            std::process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            std::io::stdout().flush().expect("Failed to flush stdout");
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => self.run(line),
                _ => break,
            }
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = crate::Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = crate::Parser::new(tokens);
        let expr = parser.parse();

        if self.had_error {
            return;
        }
        let ast_printer = AstPrinter::new();
        println!("{}", ast_printer.print(&expr.unwrap()));
    }
}

impl Default for Runner {
    fn default() -> Self {
        Runner::new()
    }
}
