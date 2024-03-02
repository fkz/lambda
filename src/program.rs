use crate::with_applications;

mod flags;
use flags::Flags;

type Instruction = u8;
pub type Program = Box<[Instruction]>;

fn is_var(instruction: Instruction) -> bool {
    instruction & 0b10000000 == 0
}

fn is_lambda(instruction: Instruction) -> bool {
    instruction & 0b10000001 == 0b10000000
}

fn is_app(instruction: Instruction) -> bool {
    instruction & 0b10000001 == 0b10000001
}

struct Var {
    value: u64,
}

impl Var {
    fn make() -> Self {
        Var { value: 0 }
    }
    fn build(&mut self, instruction: u8) -> Option<u64> {
        if !is_var(instruction) {
            return None;
        }
        self.value <<= 6;
        self.value |= (instruction & 0b00111111) as u64;
        if instruction & 0b01000000 == 0 {
            Some(std::mem::replace(&mut self.value, 0))
        } else {
            None
        }
    }

    fn instrcutions(out: &mut Vec<u8>, var: u64) {
        let mut other = Vec::new();
        let mut v = var;
        let last = (v & 63) as u8;
        v >>= 6;
        while v != 0 {
            other.push(((v & 63) | 64) as u8);
            v >>= 6;
        }
        out.extend(other.iter().rev());
        out.push(last);
    }
}

fn copy_over(target: &mut Vec<u8>, source: &Program, increase: u64) {
    let mut lambda_index: u64 = 0;
    let mut flags = Flags::make();

    let mut current_var = Var::make();

    for instruction in &**source {
        let instruction = *instruction;
        if let Some(var) = current_var.build(instruction) {
            if var >= lambda_index {
                Var::instrcutions(target, var + increase);
            } else {
                Var::instrcutions(target, var);
            }

            loop {
                let op = flags.pop_flag();
                if op == Flags::LAMBDA_FLAG {
                    lambda_index -= 1;
                } else if op == Flags::APP_FLAG_END {
                } else if op == Flags::APP_FLAG_MIDDLE {
                    break;
                } else {
                    break;
                }
            }
        } else if !is_var(instruction) {
            target.push(instruction);
            let mut instruction = instruction & 0b01111111;
            while instruction != 1 {
                let op = instruction & 1;
                instruction >>= 1;

                if op == 1 {
                    //app
                    flags.put_app();
                } else {
                    //lambda
                    flags.put_lambda();
                    lambda_index += 1;
                }
            }
        }
    }
}

fn count_ones(instruction: u8) -> u32 {
    const VALUES: [u8; 64] = [
        0x00, 0x10, 0x10, 0x21, 0x10, 0x21, 0x21, 0x32, 0x10, 0x21, 0x21, 0x32, 0x21, 0x32, 0x32,
        0x43, 0x10, 0x21, 0x21, 0x32, 0x21, 0x32, 0x32, 0x43, 0x21, 0x32, 0x32, 0x43, 0x32, 0x43,
        0x43, 0x54, 0x10, 0x21, 0x21, 0x32, 0x21, 0x32, 0x32, 0x43, 0x21, 0x32, 0x32, 0x43, 0x32,
        0x43, 0x43, 0x54, 0x21, 0x32, 0x32, 0x43, 0x32, 0x43, 0x43, 0x54, 0x32, 0x43, 0x43, 0x54,
        0x43, 0x54, 0x54, 0x65,
    ];

    (VALUES[((instruction & 127) >> 1) as usize] >> ((instruction & 1) * 4) & 7) as u32
}

#[test]
fn count_ones_correct() {
    for i in 0..=255 {
        if i == 0 || i == 128 { continue }
        let c1 = count_ones(i);
        let mut c2 = u8::count_ones(i);
        if i >= 128 { c2 -= 2 } else { c2 -= 1 };
        assert_eq!(c1, c2, "counting ones should be the same, but aren't for {}", i)
    }
}

fn consume(first_instruction: u8, program: &[u8], into: &mut Vec<u8>) -> Program {
    let mut index = 0;
    let mut remaining_applications = count_ones(first_instruction) + 1;

    while remaining_applications > 0 {
        let instruction = program[index];
        into.push(instruction);
        if instruction & 0b11000000 == 0 {
            remaining_applications -= 1;
        } else if instruction & 0b10000000 == 0b10000000 {
            remaining_applications += count_ones(instruction);
        }
        index += 1;
    }

    (program[index..]).into()
}

fn apply(program: &Program, arg: &Program, increase: u64) -> Program {
    let mut result = Vec::new();
    let mut current_increase = increase;
    let mut current_next_operator = Flags::make();
    let mut index = 1;

    let instruction = program[0];
    assert!(is_lambda(instruction));
    let instruction = 0b10000000 | ((instruction & 0b01111111) >> 1);
    if instruction != 0b10000000 && instruction != 0b10000001 {
        result.push(instruction);
        let mut bits = instruction & 0b01111111;
        while bits != 1 {
            let op = bits & 1;
            if op == 0 {
                current_increase += 1;
                current_next_operator.put_lambda();
            } else {
                current_next_operator.put_app();
            }
            bits >>= 1;
        }
    }

    'outer: loop {
        let instruction = program[index];

        if is_var(instruction) {
            let var = {
                let mut v = Var::make();
                loop {
                    if let Some(r) = v.build(program[index]) {
                        break r;
                    }
                    index += 1;
                }
            };
            if var == current_increase {
                copy_over(&mut result, arg, current_increase);
                loop {
                    let current_flag = current_next_operator.pop_flag();
                    if current_flag == Flags::LAMBDA_FLAG {
                        current_increase -= 1;
                    } else if current_flag == Flags::APP_FLAG_MIDDLE {
                        if current_next_operator.is_empty() {
                            break 'outer;
                        } else {
                            break;
                        }
                    } else if current_flag == Flags::APP_FLAG_END {
                        // do nothing
                    } else {
                        panic!("Read invalid flag");
                    }
                }
            } else {
                if var > current_increase {
                    Var::instrcutions(&mut result, var - 1);
                } else if var < current_increase {
                    Var::instrcutions(&mut result, var);
                }
                loop {
                    let current_flag = current_next_operator.pop_flag();
                    if current_flag == Flags::LAMBDA_FLAG {
                        current_increase -= 1;
                    } else if current_flag == Flags::APP_FLAG_MIDDLE {
                        if current_next_operator.is_empty() {
                            break 'outer;
                        } else {
                            break;
                        }
                    } else if current_flag == Flags::APP_FLAG_END {
                        // do nothing
                    } else {
                        panic!("Read invalid flag");
                    }
                }
            }
        } else {
            result.push(instruction);
            let mut bits = instruction & 0b01111111;
            while bits != 1 {
                let op = bits & 1;
                if op == 0 {
                    current_increase += 1;
                    current_next_operator.put_lambda();
                } else {
                    current_next_operator.put_app();
                }
                bits >>= 1;
            }
        }
        index += 1;
    }

    result.into_boxed_slice()
}

fn remove_outer_lambda(program: &Program) -> Program {
    let mut result = Vec::new();
    let instruction = program[0];
    assert!(is_lambda(instruction));
    let instruction = 0b10000000 | ((instruction & 0b01111111) >> 1);
    if instruction != 0b10000001 {
        result.push(instruction);
    }
    result.extend_from_slice(&program[1..]);
    result.into_boxed_slice()
}

fn split_application(program: &Program) -> (Program, Program) {
    let instruction = program[0];
    assert!(is_app(instruction));
    let instruction = 0b10000000 | ((instruction & 0b01111111) >> 1);
    let mut prog = Vec::new();
    if instruction != 0b10000000 && instruction != 0b10000001 {
        prog.push(instruction);
    }
    let arg = consume(instruction, &program[1..], &mut prog);

    (prog.into_boxed_slice(), arg)
}

pub struct ExecutionEnvironment {
    pub program: Program,
    pub applications: Vec<Program>,
    pub outer_lambdas: u8,
}

impl ExecutionEnvironment {
    pub fn make(program: Program) -> Self {
        ExecutionEnvironment {
            applications: Vec::new(),
            program: program,
            outer_lambdas: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        let instruction = self.program[0];
        if is_var(instruction) {
            false
        } else if is_app(instruction) {
            let (program, arg) = split_application(&self.program);
            self.program = program;
            self.applications.push(arg);
            true
        } else if is_lambda(instruction) {
            match self.applications.pop() {
                Some(param) => {
                    self.program = apply(&self.program, &param, 0);
                    true
                }
                None => {
                    self.outer_lambdas += 1;
                    self.program = remove_outer_lambda(&self.program);
                    true
                }
            }
        } else {
            false
        }
    }

    fn to_program(&self) -> Program {
        let mut result = Vec::new();
        let mut remaining_outer_lambdas = self.outer_lambdas;
        let mut remaining_applications = self.applications.len();

        while remaining_outer_lambdas >= 6 {
            remaining_outer_lambdas -= 6;
            result.push(0b11000000);
        }
        if remaining_outer_lambdas as usize + remaining_applications >= 6 {
            remaining_applications -= 6 - remaining_outer_lambdas as usize;
            result.push(0xFF ^ ((1 << remaining_outer_lambdas) - 1));
            remaining_outer_lambdas = 0;

            while remaining_applications >= 6 {
                remaining_applications -= 6;
                result.push(0xFF);
            }
        }

        if remaining_outer_lambdas > 0 || remaining_applications > 0 {
            let sum = remaining_outer_lambdas as usize + remaining_applications;
            result.push(0b10000000 | ((1 << (sum + 1)) - 1) ^ ((1 << remaining_outer_lambdas) - 1));
        }

        result.extend_from_slice(&self.program);
        for application in self.applications.iter().rev() {
            result.extend_from_slice(application);
        }
        result.into_boxed_slice()
    }
}

pub struct ExecutionEnvironmentByValue {
    pub current_program: Program,
    pub applications: Vec<Program>,
    pub before_programs: Vec<Vec<Program>>,
    pub outer_lambdas: u8,
}

impl ExecutionEnvironmentByValue {
    pub fn make(program: Program) -> Self {
        ExecutionEnvironmentByValue {
            applications: Vec::new(),
            current_program: program,
            before_programs: vec![Vec::new()],
            outer_lambdas: 0,
        }
    }

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
        for application in applications.iter().rev() {
            result.extend_from_slice(application);
        }
        result.into_boxed_slice()
    }

    pub fn step(&mut self) -> bool {
        let instruction = self.current_program[0];
        if is_var(instruction) {
            if self.before_programs.len() == 1 {
                assert!(self.before_programs[0].is_empty());
                false
            } else {
                let old_applications =
                    std::mem::replace(&mut self.applications, self.before_programs.pop().unwrap());
                let old_program = std::mem::replace(
                    &mut self.current_program,
                    self.before_programs.last_mut().unwrap().pop().unwrap(),
                );
                let prog =
                    ExecutionEnvironmentByValue::with_applications(old_program, old_applications);
                self.applications.push(prog);
                true
            }
        } else if is_app(instruction) {
            let (program, arg) = split_application(&self.current_program);
            self.current_program = arg;
            self.before_programs.last_mut().unwrap().push(program);
            let old_applications = std::mem::replace(&mut self.applications, Vec::new());
            self.before_programs.push(old_applications);
            true
        } else if is_lambda(instruction) {
            match self.applications.pop() {
                Some(param) => {
                    self.current_program = apply(&self.current_program, &param, 0);
                    true
                }
                None => {
                    if self.before_programs.len() == 1 {
                        assert!(self.before_programs[0].is_empty());
                        self.outer_lambdas += 1;
                        self.current_program = remove_outer_lambda(&self.current_program);
                        true
                    } else {
                        self.applications = self.before_programs.pop().unwrap();
                        let old_program = std::mem::replace(
                            &mut self.current_program,
                            self.before_programs.last_mut().unwrap().pop().unwrap(),
                        );
                        self.applications.push(old_program);
                        true
                    }
                }
            }
        } else {
            false
        }
    }

    fn to_program(&self) -> Program {
        assert!(self.before_programs.len() == 1);
        assert!(self.before_programs[0].is_empty());
        let mut result = Vec::new();
        let mut remaining_outer_lambdas = self.outer_lambdas;
        let mut remaining_applications = self.applications.len();

        while remaining_outer_lambdas >= 6 {
            remaining_outer_lambdas -= 6;
            result.push(0b11000000);
        }
        if remaining_outer_lambdas as usize + remaining_applications >= 6 {
            remaining_applications -= 6 - remaining_outer_lambdas as usize;
            result.push(0xFF ^ ((1 << remaining_outer_lambdas) - 1));
            remaining_outer_lambdas = 0;

            while remaining_applications >= 6 {
                remaining_applications -= 6;
                result.push(0xFF);
            }
        }

        if remaining_outer_lambdas > 0 || remaining_applications > 0 {
            let sum = remaining_outer_lambdas as usize + remaining_applications;
            result.push(0b10000000 | ((1 << (sum + 1)) - 1) ^ ((1 << remaining_outer_lambdas) - 1));
        }

        result.extend_from_slice(&self.current_program);
        for application in self.applications.iter().rev() {
            result.extend_from_slice(application);
        }
        result.into_boxed_slice()
    }
}

pub struct SimplifyEnv {
    path: Vec<(usize, ExecutionEnvironment)>,
}

impl SimplifyEnv {
    pub fn make(program: Program) -> Self {
        SimplifyEnv {
            path: vec![(0, ExecutionEnvironment::make(program))],
        }
    }

    pub fn step(&mut self) -> bool {
        let len = self.path.len();
        let current = &mut self.path[len - 1].1;

        if current.step() {
            return true;
        }

        if !current.applications.is_empty() {
            let program = current.applications[0].clone();
            self.path.push((0, ExecutionEnvironment::make(program))); // TODO Fix copying need of clone
            return true;
        }

        loop {
            let len = self.path.len();
            if len == 1 {
                return false;
            }
            let (index, current) = self.path.pop().unwrap();
            self.path[len - 2].1.applications[index] = current.to_program();
            if self.path[len - 2].1.applications.len() > index + 1 {
                self.path.push((
                    index + 1,
                    ExecutionEnvironment::make(
                        self.path[len - 2].1.applications[index + 1].clone(),
                    ),
                ));
                return true;
            }
        }
    }

    pub fn to_program(&self) -> Program {
        self.path[0].1.to_program()
    }
}

pub fn simplify(program: Program) -> Program {
    let mut simplifier = SimplifyEnv::make(program);

    while simplifier.step() {}

    simplifier.to_program()
}

pub fn simplify_by_value(program: Program) -> Program {
    let mut simplifier = ExecutionEnvironmentByValue::make(program);
    while (simplifier.step()) {}
    simplifier.to_program()
}

pub fn verify<'a>(program: &'a [u8]) -> Result<u64, &'static str> {
    let mut flags = Flags::make();
    let mut highest_free_variable = 0;
    let mut lambda_cont: u64 = 0;

    let mut var = Var::make();

    for instruction in program {
        let instruction = *instruction;

        if instruction == 128 || instruction == 129 {
            // Illegal instruction
            return Result::Err("Illegal Instruction");
        }
        if let Some(v) = var.build(instruction) {
            if v >= lambda_cont {
                highest_free_variable = u64::max(highest_free_variable, v - lambda_cont + 1);
            }
            loop {
                let op = flags.pop_flag();
                if op == Flags::LAMBDA_FLAG {
                    lambda_cont -= 1;
                } else if op == Flags::APP_FLAG_MIDDLE {
                    break;
                } else if op == Flags::APP_FLAG_END {
                } else {
                    return Result::Err("Finished, but instructions still coming");
                }
            }
        } else if !is_var(instruction) {
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
        Result::Ok(highest_free_variable)
    } else {
        Result::Err("Still more data expected")
    }
}

const VARIABLE_NAMES: [&str; 16] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "l", "m", "n", "o", "p", "q",
];

pub fn show(program: &Program) -> String {
    let mut result = String::new();
    let mut lambda_index = 0;
    let mut flags = Flags::make();

    let mut var = Var::make();

    for instruction in &**program {
        let instruction = *instruction;
        if let Some(v) = var.build(instruction) {
            if lambda_index <= v {
                let j = v - lambda_index;
                result.push_str(&format!("{}", j));
            } else {
                result.push_str(VARIABLE_NAMES[(lambda_index - 1 - v) as usize]);
            }

            loop {
                let op = flags.pop_flag();
                if op == Flags::LAMBDA_FLAG {
                    lambda_index -= 1;
                } else if op == Flags::APP_FLAG_END {
                    result.push_str(")");
                } else if op == Flags::APP_FLAG_MIDDLE {
                    break;
                } else {
                    result.push_str("|");
                    break;
                }
            }
        } else if !is_var(instruction) {
            let mut instruction = instruction & 0b01111111;
            while instruction != 1 {
                let op = instruction & 1;
                instruction >>= 1;

                if op == 1 {
                    //app
                    flags.put_app();
                    result.push_str("(");
                } else {
                    //lambda
                    flags.put_lambda();
                    result.push_str("\\");
                    result.push_str(VARIABLE_NAMES[lambda_index as usize]);
                    lambda_index += 1;
                }
            }
        }
    }

    assert!(flags.is_empty());
    result
}

pub fn show_executor(executor: &ExecutionEnvironment) -> String {
    let mut result = show(&executor.program);
    if executor.outer_lambdas > 0 {
        result = format!("({})=>{}", executor.outer_lambdas - 1, result);
    }
    for program in executor.applications.iter() {
        result.push_str("<");
        result.push_str(&show(program));
    }
    result
}

pub fn show_executor_by_value(executor: &ExecutionEnvironmentByValue) -> String {
    let mut result = String::new();
    if executor.outer_lambdas > 0 {
        result = format!("({})=>", executor.outer_lambdas - 1);
    }
    if !executor.before_programs[0].is_empty() {
        assert_eq!(executor.before_programs[0].len(), 1);
        result.push_str(&show(&executor.before_programs[0][0]));
    }
    for index in 1..executor.before_programs.len() {
        result.push_str("|");
        for p in executor.before_programs[index].iter() {
            result.push_str("<");
            result.push_str(&show(p));
        }
    }
    //if !executor.before_programs[0].is_empty() {
    result.push_str("::");
    //}
    result.push_str(&show(&executor.current_program));
    for program in executor.applications.iter() {
        result.push_str("<");
        result.push_str(&show(program));
    }

    result
}

pub fn simplify_debug(program: Program) -> Program {
    println!("Verified: {:?}", verify(&program));
    println!("Start: {}", show(&program));
    let mut simplifier = SimplifyEnv::make(program);
    while simplifier.step() {
        println!("Step: {}", show_executor(&simplifier.path[0].1));
        for index in simplifier.path.as_slice()[1..].iter() {
            println!("  {} {}", index.0, show_executor(&index.1))
        }
    }

    simplifier.to_program()
}

pub fn simplify_by_value_debug(program: Program) -> Program {
    let mut simplifier = ExecutionEnvironmentByValue::make(program);
    while simplifier.step() {
        println!("Step: {}", show_executor_by_value(&simplifier));
    }
    simplifier.to_program()
}

pub fn simplify_generic(program: Program, show_steps: bool, by_value: bool) -> Program {
    match (show_steps, by_value) {
        (false, false) => simplify(program),
        (true, false) => simplify_debug(program),
        (false, true) => simplify_by_value(program),
        (true, true) => simplify_by_value_debug(program),
    }
}
