pub mod example_interact_programs;
mod example_programs;
mod human_readable;
mod interact;
mod program;
mod simple_env;

use program::{simplify, simplify_debug};
use simple_env::Env;

use crate::{interact::Environment, program::show};

fn main() {
    let mut index = 1;
    let mut show_steps = false;
    let mut interactive = None;
    loop {
        let flag = std::env::args().nth(index).expect("no path given");
        if (!flag.starts_with("--")) {
            break;
        }
        match flag.as_str() {
            "--show-steps" => show_steps = true,
            "--interactive" => interactive = Some(Env::make()),
            other => panic!("Unknown flag {}", other)
        }
        index += 1;
    }
    
    let path = std::env::args().nth(index).expect("no path given");

    let program = human_readable::parse_file(&path);

    if let Some(mut env) = interactive {
        interact::interact(&mut env, &program);
    } else {
        println!("Input: {}", show(&program));
        let simplified = {
            if show_steps {
                simplify_debug(program)
            } else {
                simplify(program)
            }
        };
        println!("Simplified: {}", show(&simplified));
    }
}
