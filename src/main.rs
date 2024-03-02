use lambda_calculus as lib;

fn main() {
    let mut index = 1;
    let mut show_steps = false;
    let mut show_program = true;
    let mut interactive = None;
    let mut prettify = lib::pretty::nothing();
    let mut show_hex = false;
    let mut by_value = false;

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
            other => panic!("Unknown flag {}", other),
        }
    };

    let program = lib::parse_arguments(&path, &prettify, arg_iterator.collect());

    if let Some(mut env) = interactive {
        lib::execute_interactive(&mut env, program, show_steps, by_value)
    } else {
        let result = lib::execute(program, show_steps, show_program, show_hex, by_value);

        println!("Result: {:?}", prettify.program_to_string(&result).unwrap());
    }
}
