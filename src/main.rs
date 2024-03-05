use lambda_calculus as lib;
use lib::memory_representation;

fn main() {
    let mut index = 1;
    let mut show_steps = false;
    let mut show_program = true;
    let mut interactive = None;
    let mut prettify = lib::pretty::nothing();
    let mut show_hex = false;
    let mut by_value = false;
    let mut new = false;

    let mut arg_iterator = std::env::args();
    let _program_name = arg_iterator.next();

    let path = loop {
        let flag = arg_iterator.next().expect("no path given");
        if !flag.starts_with("--") {
            break flag;
        }
        match flag.as_str() {
            "--show-steps" => show_steps = true,
            "--interactive" => interactive = Some(lib::simple_env::Env::make()),
            "--number" => {
                prettify = lib::pretty::number();
                show_program = false;
            }
            "--show-program" => show_program = true,
            "--show-hex" => show_hex = true,
            "--by-value" => by_value = true,
            "--new" => new = true,
            other => panic!("Unknown flag {}", other),
        }
    };

    let program = lib::parse_arguments(&path, &prettify, arg_iterator.collect());


    if new {
        let instr = memory_representation::parse_program(&program).unwrap();
        if show_program {
            println!("Program: {}", lib::pretty::nothing().program_to_string(&instr.to_opcode()).unwrap());
        }
        let instr = memory_representation::parse_program(&program).unwrap();
        let mut executor = memory_representation::Executor::new(instr);
        let mut step = 0;
        loop {
            if show_steps { println!("Step: {}", executor.show()) };
            let (e, cont) = executor.step();
            executor = e;
            if !cont {
                break;
            }
            step += 1;
        }
        let program = executor.to_program().to_opcode();
        if show_program {
            println!("Simplified: {}", lib::pretty::nothing().program_to_string(&program).unwrap());
        }
        println!("Steps: {}", step);
    
        println!("Result: {}", prettify.program_to_string(&program).unwrap());
    } else if let Some(mut env) = interactive {
        lib::execute_interactive(&mut env, program, show_steps, by_value)
    } else {
        let result = lib::execute(program, show_steps, show_program, show_hex, by_value);

        println!("Result: {:?}", prettify.program_to_string(&result).unwrap());
    }
}
