use super::program::flags::Flags;
use std::{mem::ManuallyDrop, rc::Rc};

#[derive(Debug, Clone)]
pub enum Program {
    Lambda(Box<Program>),
    App(Box<Program>, Vec<Program>),
    Var(u64),
    GlobalVar(u64),
    Reference(Rc<Program>),
}

struct InstructionFlags {
    flags: u8,
    bit: u8,
}

impl InstructionFlags {
    fn new() -> InstructionFlags {
        InstructionFlags { flags: 0, bit: 1 }
    }

    fn push(&mut self, result: &mut Vec<u8>, flag: bool) {
        if flag {
            self.flags |= self.bit;
        }
        self.bit <<= 1;
        if self.bit == 0b01000000 {
            result.push(self.flags | 0b11000000);
            self.flags = 0;
            self.bit = 1;
        }
    }

    fn flush(&mut self, result: &mut Vec<u8>) {
        if self.bit > 1 {
            result.push(self.flags | 0b10000000 | self.bit);
            self.flags = 0;
            self.bit = 1;
        }
    }
}

impl Program {
    pub fn to_opcode(self) -> super::program::Program {
        let mut result = Vec::new();
        let mut app_stack = Vec::new();
        let mut current = (self, 0);
        let mut instruction_flags = InstructionFlags::new();

        loop {
            match current.0 {
                Program::Lambda(p) => {
                    instruction_flags.push(&mut result, false);
                    current = (*p, current.1 + 1);
                }
                Program::App(f, args) => {
                    let len = args.len();
                    app_stack.extend(args.into_iter().rev().map(|a| (a, current.1)));
                    current.0 = *f;
                    for _ in 0..len {
                        instruction_flags.push(&mut result, true);
                    }
                }
                Program::Var(v) => {
                    instruction_flags.flush(&mut result);
                    super::program::Var::instrcutions(&mut result, v);

                    if let Some(next) = app_stack.pop() {
                        current = next;
                    } else {
                        break;
                    }
                }
                Program::GlobalVar(v) => {
                    current.0 = Program::Var(current.1 + v);
                    //panic!("No global vars supported {}", v);
                }
                Program::Reference(p) => {
                    current.0 = Rc::try_unwrap(p).unwrap_or_else(|p| (*p).clone());
                }
            }
        }

        result.into_boxed_slice()
    }
}

pub fn parse_program(input: &[u8]) -> Result<Program, &'static str> {
    let mut flags = Flags::make();
    let mut highest_free_variable = 0;
    let mut lambda_cont: u64 = 0;

    let mut var = super::program::Var::make();

    let mut result = Vec::new();

    for instruction in input {
        let instruction = *instruction;

        if instruction == 128 || instruction == 129 {
            // Illegal instruction
            return Result::Err("Illegal Instruction");
        }
        if let Some(v) = var.build(instruction) {
            if v >= lambda_cont {
                highest_free_variable = u64::max(highest_free_variable, v - lambda_cont + 1);
            }

            let mut current = if v >= lambda_cont {
                Program::GlobalVar(v - lambda_cont)
            } else {
                Program::Var(v)
            };

            loop {
                let op = flags.pop_flag();
                if op == Flags::LAMBDA_FLAG {
                    lambda_cont -= 1;
                    current = Program::Lambda(Box::new(current));
                } else if op == Flags::APP_FLAG_MIDDLE {
                    result.push(current);
                    break;
                } else if op == Flags::APP_FLAG_END {
                    let f = result.pop().unwrap();
                    current = Program::App(Box::new(f), vec![current]);
                } else {
                    return Result::Err("Finished, but instructions still coming");
                }
            }
        } else if instruction & 0b10000000 != 0 {
            if flags.is_empty() {
                return Result::Err("Finished, but new instructions still coming");
            }
            let mut instruction = instruction & 0b01111111;
            while instruction != 1 {
                let op = instruction & 1;
                instruction >>= 1;
                if op == 0 {
                    flags.put_lambda();
                    lambda_cont += 1;
                } else {
                    flags.put_app();
                }
            }
        }
    }
    if flags.is_empty() {
        assert_eq!(result.len(), 1);
        Result::Ok(result.pop().unwrap())
    } else {
        Result::Err("Still more data expected")
    }
}

#[derive(Debug)]
pub struct Executor {
    current: Program,
    app_stack: Vec<Program>,
    previous: Vec<(Program, Vec<Program>)>,
    lambdas: u64,
}

impl Executor {
    pub fn new(program: Program) -> Executor {
        Executor {
            current: program,
            app_stack: Vec::new(),
            previous: Vec::new(),
            lambdas: 0,
        }
    }

    pub fn to_program(self) -> Program {
        assert_eq!(self.previous.len(), 0);
        let mut result = Executor::replace_glob(
            Program::App(Box::new(self.current), self.app_stack),
            self.lambdas,
        );
        for _ in 0..self.lambdas {
            result = Program::Lambda(Box::new(result));
        }
        result
    }

    fn replace(f: Program, app: Program) -> Program {
        let rc = Rc::new(app);
        let mut f = f;

        let mut to_do = Vec::new();
        to_do.push((&mut f, 0));

        while let Some((mut current, depth)) = to_do.pop() {
            match current {
                Program::Lambda(p) => {
                    to_do.push((p, depth + 1));
                }
                Program::App(f, args) => {
                    to_do.push((f, depth));
                    for arg in args {
                        to_do.push((arg, depth));
                    }
                }
                Program::Var(v) => {
                    if *v == depth {
                        *current = Program::Reference(rc.clone());
                    }
                }
                Program::GlobalVar(v) => {
                    // Do nothing
                }
                Program::Reference(p) => {
                    // Do nothing
                }
            }
        }

        f
    }

    fn replace_glob(f: Program, glob_count: u64) -> Program {
        let mut f = f;

        let mut to_do = Vec::new();
        to_do.push((&mut f, 0));

        while let Some((mut current, depth)) = to_do.pop() {
            match current {
                Program::Lambda(p) => {
                    to_do.push((p, depth + 1));
                }
                Program::App(f, args) => {
                    to_do.push((f, depth));
                    for arg in args {
                        to_do.push((arg, depth));
                    }
                }
                Program::Var(v) => {
                    if *v >= depth {
                        panic!("unused var {} >= {}", v, depth);
                    }
                }
                l @ Program::GlobalVar(_) => {
                    if let Program::GlobalVar(q) = l.clone() {
                        *l = Program::Var(glob_count - q - 1);
                    } else {
                        panic!("global var not global var");
                    }
                }
                r @ Program::Reference(_) => {
                    if let Program::Reference(q) = r.clone() {
                        *r = (*q).clone();
                        to_do.push((r, depth));
                    } else {
                        panic!("ref not ref");
                    }
                }
            }
        }

        f
    }

    pub fn step(mut self) -> (Self, bool) {
        match self.current {
            Program::Lambda(p) => {
                if let Some(app) = self.app_stack.pop() {
                    self.current = Executor::replace(*p, app);
                    (self, true)
                } else {
                    if let Some((previous_current, previous_app)) = self.previous.pop() {
                        self.app_stack = previous_app;
                        self.app_stack.push(Program::Lambda(p));
                        self.current = previous_current;
                        (self, true)
                    } else {
                        self.current = Executor::replace(*p, Program::GlobalVar(self.lambdas));
                        self.lambdas += 1;
                        (self, true)
                    }
                }
            }
            Program::App(f, mut args) => {
                if let Some(arg) = args.pop() {
                    self.previous.push((Program::App(f, args), self.app_stack));
                    self.app_stack = Vec::new();
                    self.current = arg;
                    (self, true)
                } else {
                    self.current = *f;
                    (self, true)
                }
            }
            Program::Var(v) => {
                panic!("Unbound var {}", v);
            }
            Program::GlobalVar(v) => {
                if let Some((previous_f, previous_app)) = self.previous.pop() {
                    let mut current = self.current;
                    if self.app_stack.len() > 0 {
                        current = Program::App(Box::new(current), self.app_stack);
                    }
                    self.app_stack = previous_app;
                    self.app_stack.push(current);
                    self.current = previous_f;
                    (self, true)
                } else {
                    (self, false)
                }
            }
            Program::Reference(p) => match Rc::try_unwrap(p) {
                Ok(p) => {
                    self.current = p;
                    (self, true)
                }
                Err(p) => {
                    self.current = (*p).clone();
                    (self, true)
                }
            },
        }
    }

    pub fn show(&self) -> String {
        let mut result = super::show(&self.current.clone().to_opcode());
        for app in self.app_stack.iter().rev() {
            result.push_str("<");
            result.push_str(&super::show(&app.clone().to_opcode()));
        }
        for (c, c_app) in self.previous.iter() {
            result.push_str("\n");
            result.push_str(&super::show(&c.clone().to_opcode()));
            for app in c_app.iter().rev() {
                result.push_str("<");
                result.push_str(&super::show(&app.clone().to_opcode()));
            }
        }
        result
    }
}
