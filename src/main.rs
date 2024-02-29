pub mod example_interact_programs;
mod example_programs;
mod human_readable;
mod interact;
mod program;
mod simple_env;
mod pretty;

use pretty::Pretty;
use program::{Program, simplify, simplify_debug};
use simple_env::Env;

use crate::{interact::Environment, program::show};

fn with_applications(p: Program, applications: Vec<Program>) -> Program {
    if applications.is_empty() {
        return p;
    }
    let mut result = Vec::new();
    let mut count = applications.len();
    while count > 6 {
        count -= 6;
        result.push(0xFF);
    }
    result.push(0x80 | ((1 << (count + 1)) - 1));
    result.extend_from_slice(&p);
    for application in applications {
        result.extend_from_slice(&application);
    }
    result.into_boxed_slice()
}

fn main() {
    let mut index = 1;
    let mut show_steps = false;
    let mut show_program = true;
    let mut interactive = None;
    let mut prettify = None;
    let mut show_hex = false;
    loop {
        let flag = std::env::args().nth(index).expect("no path given");
        if (!flag.starts_with("--")) {
            break;
        }
        match flag.as_str() {
            "--show-steps" => show_steps = true,
            "--interactive" => interactive = Some(Env::make()),
            "--number" => { prettify = Some(pretty::Number); show_program = false; },
            "--show-program" => show_program = true,
            "--show-hex" => show_hex = true,
            other => panic!("Unknown flag {}", other)
        }
        index += 1;
    }
    
    let path = std::env::args().nth(index).expect("no path given");

    let remaining = std::env::args().len() - index - 1;
    let mut applications = Vec::new();
    if remaining > 0 {
        let prettify = prettify.as_ref().expect("when using arguments, use --number to parse them");
        while (applications.len() < remaining) {
            let arg = std::env::args().nth(index + applications.len() + 1).unwrap();
            let program = prettify.string_to_program(&arg)
                .expect(format!("Expected {}, but got {}", prettify.name(), arg).as_str());
            applications.push(program);
        }
    }

    let program = human_readable::parse_file(&path);
    let program = with_applications(program, applications);

    println!("Verify {:?}", program::verify(&program));

    if let Some(mut env) = interactive {
        interact::interact(&mut env, &program);
    } else {
        if show_program {
            println!("Input: {}", show(&program));
        }
        let simplified = {
            if show_steps {
                simplify_debug(program)
            } else {
                simplify(program)
            }
        };
        if show_program {
            println!("Simplified: {}", show(&simplified));
        }
        if show_hex {
            println!("Simplified: {}", hex::encode(&simplified));
        }
        if let Some(p) = prettify {
            let result = p.program_to_string(&simplified)
                .expect(format!("Couldn't parse result of program as {}", p.name()).as_str());
            println!("Result: {}", result);
        }
    }
}
