mod compare;
pub mod example_interact_programs;
mod example_programs;
mod human_readable;
mod interact;
pub mod memory_representation;
pub mod pretty;
mod program;
pub mod simple_env;

use interact::{Request, Response};
use pretty::Pretty;
use program::{simplify_debug, simplify_generic};
use simple_env::Env;

pub type Program = program::Program;

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

pub fn parse_arguments<T: Clone + std::fmt::Display>(
    program_path: &str,
    prettifier: &Box<dyn Pretty<T>>,
    arguments: Vec<T>,
) -> Program {
    let program = human_readable::parse_file(&program_path);

    let mut applications = Vec::with_capacity(arguments.len());
    for arg in arguments {
        let program = prettifier
            .string_to_program(arg.clone())
            .expect(format!("Expected {}, but got {}", prettifier.name(), arg).as_str());
        applications.push(program);
    }

    with_applications(program, applications)
}

pub fn execute_interactive<
    Env: Environment<Request, Response>,
    Request: interact::Request,
    Response: interact::Response,
>(
    env: &mut Env,
    program: Program,
    show_steps: bool,
    by_value: bool,
) {
    //println!("Verify {:?}", program::verify(&program));
    interact::interact(env, &program, by_value, show_steps);
}

pub fn execute(
    program: Program,
    show_steps: bool,
    show_program: bool,
    show_hex: bool,
    by_value: bool,
    new: bool,
) -> Program {
    //println!("Verify {:?}", program::verify(&program));

    if show_program {
        println!("Input: {}", show(&program));
    }

    let simplified = if new {
        let instr = memory_representation::parse_program(&program).unwrap();
        let mut executor = memory_representation::Executor::new(instr);
        let mut step = 0;
        loop {
            if !by_value {
                panic!("Not implemented");
            }
            if show_steps {
                println!("Step: {}", executor.show())
            };
            let (e, cont) = executor.step();
            executor = e;
            if !cont {
                break;
            }
            step += 1;
        }
        if show_program {
            println!("Steps: {}", step);
        }
        executor.to_program().to_opcode()
    } else {
        simplify_generic(program, show_steps, by_value)
    };
    if show_program {
        println!("Simplified: {}", show(&simplified));
    }
    if show_hex {
        println!("Simplified: {}", hex::encode(&simplified));
    }

    simplified
}
