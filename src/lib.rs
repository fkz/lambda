mod compare;
pub mod example_interact_programs;
mod example_programs;
pub mod gc_mem;
mod human_readable;
mod interact;
pub mod memory_representation;
pub mod pretty;
mod program;
pub mod simple_env;

use gc_mem::Allocator;
use pretty::Pretty;
use program::simplify_generic;

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
    new_gc: bool,
    step_count: &mut u64,
    gc_mem_alloc: Option<&mut gc_mem::alloc_impl::Allocator>,
) -> Program {
    //println!("Verify {:?}", program::verify(&program));

    if show_program {
        println!("Input: {}", show(&program));
    }

    let simplified = if new {
        let instr = memory_representation::parse_program(&program).unwrap();

        if new_gc {
            let a = gc_mem_alloc.unwrap_or_else(|| gc_mem::alloc_impl::get_allocator());
            let mut executor = gc_mem::Executor::<gc_mem::alloc_impl::Allocator>::new(
                a.to_long(&a.new(&gc_mem::from_mem_rep(a, &instr))),
            );
            let mut step = 0;
            loop {
                if !by_value {
                    panic!("Not implemented");
                }
                if show_steps {
                    println!("Step"); //, executor.show())
                };
                let (e, cont) = executor.step(&a);
                executor = e;
                if !cont {
                    break;
                }
                step += 1;
                if step % 65536 == 0 {
                    //a.collect_garbage(true);
                }
            }
            *step_count = step;
            a.collect_garbage(true);
            gc_mem::to_mem_rep(a, a.deref_short(&executor.to_program(a))).to_opcode()
        } else {
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
            *step_count = step;
            executor.to_program().to_opcode()
        }
    } else {
        simplify_generic(program, show_steps, by_value, step_count)
    };
    if show_program {
        println!("Simplified: {}", show(&simplified));
    }
    if show_hex {
        println!("Simplified: {}", hex::encode(&simplified));
    }

    simplified
}
