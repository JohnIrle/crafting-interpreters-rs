pub struct Runner {
    had_error: bool
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            had_error: false
        }
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
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => self.run(line),
                _ => break
            }
        }
    }

    fn run (&mut self, source: String) {
        let mut scanner = crate::Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
        self.had_error = true;
    }

}
