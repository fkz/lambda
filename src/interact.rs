use crate::program::{simplify, verify, ExecutionEnvironment, Program};

pub trait Request: Sized {
    fn from_program(program: &Program) -> Option<Self>;
}

pub trait Response: Sized {
    fn to_program(self) -> Program;
}

pub trait Environment<Req, Res>
where
    Req: Request,
    Res: Response,
{
    fn make() -> Self;

    fn request(&mut self, request: Req);
    fn panic(&mut self, panic_info: PanicInfo);

    fn next_response(&mut self) -> Option<Res>;

    fn finished(&self) -> bool;
}

#[derive(Debug)]
pub enum PanicInfo {
    InvalidProgram(&'static str),
    ProgramContainsFreeVariables(u8),
    InvalidRequest,
    InvalidState,
}

fn apply_free(prog: &[u8], arg: &[u8]) -> Program {
    let mut result = Vec::with_capacity(prog.len() + arg.len() + 1);
    result.push(0x85);
    result.extend_from_slice(prog);
    result.extend_from_slice(arg);
    result.into_boxed_slice()
}

fn apply1(prog: &[u8], arg: &[u8]) -> Program {
    let mut result = Vec::with_capacity(prog.len() + arg.len() + 1);
    result.push(0x83);
    result.extend_from_slice(prog);
    result.extend_from_slice(arg);
    result.into_boxed_slice()
}

fn add_response<Res: Response>(program: Program, response: Res) -> Program {
    let mut arg = Vec::new();
    arg.push(0x8E);
    arg.push(0x00);
    arg.extend_from_slice(&response.to_program());
    arg.push(0x83);
    arg.push(0x01);
    arg.push(0x00);
    apply_free(&program, &arg.into_boxed_slice())
}

fn lambda(program: &[u8]) -> Program {
    let mut result = Vec::new();
    result.push(0x82);
    result.extend_from_slice(program);
    result.into_boxed_slice()
}

pub fn interact<Env: Environment<Req, Res>, Req: Request, Res: Response>(
    env: &mut Env,
    program: &[u8],
) {
    match crate::program::verify(program) {
        Result::Err(err) => {
            env.panic(PanicInfo::InvalidProgram(err));
            return;
        }
        Ok(0) => {}
        Ok(free_vars) => {
            env.panic(PanicInfo::ProgramContainsFreeVariables(free_vars));
            return;
        }
    }

    let stream = &[0x86, 0x01, 0x00];
    let mut program_state = apply1(program, stream);

    loop {
        while let Some(resp) = env.next_response() {
            program_state = add_response(program_state, resp);
        }

        if let Result::Err(err) = verify(&program_state) {
            println!("Error while verifying program correctness: {}", err);
            return;
        }

        let mut executor = ExecutionEnvironment::make(program_state);
        while executor.step() {}

        //println!("After simplification: {}", crate::program::show_executor(&executor));

        if executor.outer_lambdas != 1
            || *executor.program != [0x00]
            || executor.applications.len() != 2
        {
            env.panic(PanicInfo::InvalidState);
            return;
        }

        let request = simplify(executor.applications.remove(1));

        match Req::from_program(&request) {
            Some(req) => env.request(req),
            None => {
                env.panic(PanicInfo::InvalidRequest);
                return;
            }
        }

        if env.finished() {
            return;
        }

        program_state = lambda(&executor.applications.remove(0));
    }
}
