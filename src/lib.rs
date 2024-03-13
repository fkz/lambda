mod compare;
mod direct_executor;
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

#[derive(Clone, Copy)]
pub enum ExecutionModel {
    Default,
    ByValue,
    NewByValue,
    NewGcByValue,
    DirectByValue,
}

impl ExecutionModel {
    fn is_new(self) -> bool {
        match self {
            ExecutionModel::NewByValue
            | ExecutionModel::NewGcByValue
            | ExecutionModel::DirectByValue => true,
            _ => false,
        }
    }

    pub fn is_by_value(self) -> bool {
        match self {
            ExecutionModel::ByValue
            | ExecutionModel::NewByValue
            | ExecutionModel::NewGcByValue
            | ExecutionModel::DirectByValue => true,
            _ => false,
        }
    }

    pub fn with_by_value(self) -> ExecutionModel {
        match self {
            ExecutionModel::Default => ExecutionModel::ByValue,
            _ => self,
        }
    }

    pub fn with_new(self) -> ExecutionModel {
        match self {
            ExecutionModel::Default | ExecutionModel::ByValue => ExecutionModel::NewByValue,
            _ => self,
        }
    }
}

pub fn execute(
    program: Program,
    show_steps: bool,
    show_program: bool,
    show_hex: bool,
    execution_model: ExecutionModel,
    step_count: &mut u64,
    gc_mem_alloc: Option<&mut gc_mem::alloc_impl::Allocator>,
) -> Program {
    //println!("Verify {:?}", program::verify(&program));

    if show_program {
        println!("Input: {}", show(&program));
    }

    let simplified = if execution_model.is_new() {
        let instr = memory_representation::parse_program(&program).unwrap();

        if let ExecutionModel::NewGcByValue = execution_model {
            let a = gc_mem_alloc.unwrap_or_else(|| gc_mem::alloc_impl::get_allocator());
            let mut executor = gc_mem::Executor::<gc_mem::alloc_impl::Allocator>::new(
                a.to_long(&a.new(&gc_mem::from_mem_rep(a, &instr))),
            );
            let mut step = 0;
            loop {
                if !execution_model.is_by_value() {
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
                a.collect_garbage(false, true);
            }
            *step_count = step;
            let result = gc_mem::to_mem_rep(a, a.deref_short(&executor.to_program(a))).to_opcode();
            a.collect_garbage(true, true);
            result
        } else if let ExecutionModel::DirectByValue = execution_model {
            let mut executor = direct_executor::from_rep(instr);

            let mut step = 0;
            loop {
                if !execution_model.is_by_value() {
                    panic!("Not implemented");
                }
                if show_steps {
                    println!("Step: {}", executor.show());
                };
                let cont = executor.step();
                if !cont {
                    break;
                }
                step += 1;
            }
            *step_count = step;
            direct_executor::to_rep(executor).to_opcode()
        } else {
            let mut executor = memory_representation::Executor::new(instr);
            let mut step = 0;
            loop {
                if !execution_model.is_by_value() {
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
        simplify_generic(
            program,
            show_steps,
            execution_model.is_by_value(),
            step_count,
        )
    };
    if show_program {
        println!("Simplified: {}", show(&simplified));
    }
    if show_hex {
        println!("Simplified: {}", hex::encode(&simplified));
    }

    simplified
}
