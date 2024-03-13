use lambda_calculus as lib;
use lib::ExecutionModel;

fn main() {
    let mut show_steps = false;
    let mut show_program = false;
    let mut interactive = None;
    let mut prettify = lib::pretty::nothing();
    let mut show_hex = false;
    let mut execution_model = lib::ExecutionModel::Default;
    let mut show_step_count = true;

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
            "--by-value" => execution_model = execution_model.with_by_value(),
            "--new" => execution_model = execution_model.with_new(),
            "--new-gc" => execution_model = ExecutionModel::NewGcByValue,
            "--direct" => execution_model = ExecutionModel::DirectByValue,
            "--no-show-step-count" => show_step_count = false,
            other => panic!("Unknown flag {}", other),
        }
    };

    let program = lib::parse_arguments(&path, &prettify, arg_iterator.collect());

    if let Some(mut env) = interactive {
        lib::execute_interactive(&mut env, program, show_steps, execution_model.is_by_value())
    } else {
        let mut step_count = 0;
        let result = lib::execute(
            program,
            show_steps,
            show_program,
            show_hex,
            execution_model,
            &mut step_count,
            None,
        );
        if show_step_count {
            println!("Step count: {}", step_count);
        }

        println!("Result: {}", prettify.program_to_string(&result).unwrap());
    }
}
