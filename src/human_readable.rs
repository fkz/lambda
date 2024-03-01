// Compile (more) human-readable files

use crate::program::Program;
use std::collections::HashMap;

struct State {
    chunks: Vec<(String, Program)>,
    main: Option<Program>,
    error: Option<(String, CompileError)>,
    location: String,
}

#[derive(Debug)]
enum CompileError {
    InvalidCode(&'static str),
    FreeVariables(u64),
    DuplicateName(String),
}

impl State {
    fn add_error(&mut self, error: CompileError) {
        self.error = Some((self.location.clone(), error));
    }

    fn verify(&mut self, program: &[u8]) {
        match crate::program::verify(program) {
            Err(e) => self.add_error(CompileError::InvalidCode(e)),
            Ok(0) => (),
            Ok(free) => self.add_error(CompileError::FreeVariables(free)),
        }
    }

    fn compile(&self, code: &str) -> Program {
        let mut c = String::from(code);
        for (name, chunk) in self.chunks.iter().rev() {
            c = c.replace(name, &hex::encode(chunk))
        }
        c.retain(|c| !c.is_whitespace());
        hex::decode(c).unwrap().into_boxed_slice()
    }

    fn add_chunk(&mut self, name: &str, code: Program) {
        self.verify(&code);
        let index = match self.chunks.binary_search_by_key(&name.len(), |x| x.0.len()) {
            Err(index) => index,
            Ok(index) => index,
        };

        self.chunks.insert(index, (String::from(name), code));
    }

    fn add_main(&mut self, code: Program) {
        self.verify(&code);
        self.main = Some(code);
    }

    fn make() -> Self {
        State {
            chunks: Vec::new(),
            main: None,
            error: None,
            location: "Start".to_string(),
        }
    }

    fn process_line(&mut self, line: &str) {
        self.location = String::from(line);
    }
}

pub fn parse_file(path: &str) -> Program {
    let mut state = State::make();

    for line in std::fs::read_to_string(path).unwrap().split('\n') {
        if line.is_empty() {
            continue;
        }
        if line == "clear errors" {
            state.error = None;
            continue;
        }
        state.process_line(line);
        if line.starts_with("//") {
            continue;
        }
        let b: Box<[&str]> = line.split('=').collect();
        match *b {
            [code] => state.add_main(state.compile(code)),
            [name, code] => state.add_chunk(name, state.compile(code)),
            _ => panic!("Wrong program"),
        }
    }

    if let Some(s) = state.error {
        panic!("Error while parsing file: {:?}", s)
    }

    state.main.unwrap()
}
