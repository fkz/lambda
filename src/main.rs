
type Instruction = u8;
type Program = Box<[Instruction]>;


struct Flags {
    flags: u128,
    more_flags: Vec<u128>
}

impl Flags {
    fn make() -> Self {
        Flags {
            flags: 0,
            more_flags: Vec::new()
        }
    }

    fn is_empty(&self) -> bool {
        self.flags == 0 && self.more_flags.is_empty()
    }

    fn extend(&mut self, bits: usize) {
        if self.more_flags.len() == 0 {
            if self.flags & (0xF << 124) == 0 {
                self.flags <<= bits;
                return;
            } else {
                self.more_flags.push(0);
            }
        } else if self.more_flags[self.more_flags.len()-1] & (0xF << 124) != 0 {
            self.more_flags.push(0);
        }
        let mut index = self.more_flags.len()-1;
        while index >= 1 {
            self.more_flags[index] <<= bits;
            let previous = self.more_flags[index - 1];
            self.more_flags[index] |= (((1 << bits) - 1) << (128 - bits) & previous) >> (128 - bits);
            index -= 1;
        }
        self.more_flags[0] <<= bits;
        self.more_flags[0] |= (((1 << bits) - 1) << (128 - bits) & self.flags) >> (128 - bits);
        self.flags <<= bits;
    }

    fn shrink(&mut self, bits: usize) {
        self.flags >>= bits;
        if self.more_flags.len() > 0 {
            self.flags |= (self.more_flags[0] & ((1 << bits) - 1)) << (128 - bits);
            self.more_flags[0] >>= bits;
            let mut index = 1;
            while index < self.more_flags.len() {
                self.more_flags[index - 1] |= (self.more_flags[index] & ((1 << bits) - 1)) << (128 - bits);
                self.more_flags[index] >>= bits;
                index += 1;
            }
            if self.more_flags[self.more_flags.len()-1] == 0 {
                self.more_flags.remove(self.more_flags.len()-1);
            }
        }
    }

    fn put_lambda(&mut self) {
        self.extend(2);
        self.flags |= 1;
    }

    fn put_app(&mut self) {
        self.extend(4);
        self.flags |= 14;
    }

    fn pop_flag(&mut self) -> u8 {
        let result = (self.flags & 3) as u8;
        self.shrink(2);
        result
    }

    const LAMBDA_FLAG: u8 = 1;
    const APP_FLAG_MIDDLE: u8 = 2;
    const APP_FLAG_END: u8 = 3;
}


fn is_var(instruction: Instruction) -> bool {
    instruction & 0b10000000 == 0
}

fn is_lambda(instruction: Instruction) -> bool {
    instruction & 0b10000001 == 0b10000000
}

fn is_app(instruction: Instruction) -> bool {
    instruction & 0b10000001 == 0b10000001
}

fn copy_over(target: &mut Vec<u8>, source: &Program, increase: u8) {
    let mut lambda_index = 0;
    let mut flags = Flags::make();
    
    for instruction in &**source {
        let instruction = *instruction;
        if is_var(instruction) {
            if instruction >= lambda_index {
                target.push(instruction + increase);
            } else {
                target.push(instruction);
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
        } else {
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


fn consume(first_instruction: u8, program: &[u8], into: &mut Vec<u8>) -> Program {
    let mut index = 0;
    let mut remaining_applications = u8::count_ones(first_instruction) - 1;

    while remaining_applications > 0 {
        let instruction = program[index];
        into.push(instruction);
        if instruction & 0b10000000 == 0 {
            remaining_applications -= 1;
        } else {
            remaining_applications += u8::count_ones(instruction) - 2;
        }
        index += 1;
    }

    (program[index..]).into()
}

fn apply(program: &Program, arg: &Program, increase: u8) -> Program {
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

        if instruction == current_increase {
            copy_over(&mut result, arg, current_increase);
            loop {
                let current_flag = current_next_operator.pop_flag();
                if current_flag == Flags::LAMBDA_FLAG {
                    current_increase -= 1;
                } else if current_flag == Flags::APP_FLAG_MIDDLE {
                    break;
                } else if current_flag == Flags::APP_FLAG_END {
                    // do nothing
                } else {
                    break 'outer;
                }
            }
        } else if is_var(instruction) {
            if instruction > current_increase {
                result.push(instruction - 1);
            } else if instruction < current_increase {
                result.push(instruction);
            }
            loop {
                let current_flag = current_next_operator.pop_flag();
                if current_flag == Flags::LAMBDA_FLAG {
                    current_increase -= 1;
                } else if current_flag == Flags::APP_FLAG_MIDDLE {
                    break;
                } else if current_flag == Flags::APP_FLAG_END {
                    // do nothing
                } else {
                    break 'outer;
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


struct ExecutionEnvironment {
    program: Program,
    applications: Vec<Program>,
    outer_lambdas: u8
}

impl ExecutionEnvironment {
    fn make(program: Program) -> Self {
        ExecutionEnvironment {
            applications: Vec::new(),
            program: program,
            outer_lambdas: 0
        }
    }

    fn step(&mut self) -> bool {
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

        loop {
            let mut next_instruction: u8 = 1;
            while next_instruction < 0b01000000 {
                if remaining_applications > 0 {
                    remaining_applications -= 1;
                    next_instruction <<= 1;
                    next_instruction |= 1;
                } else if remaining_outer_lambdas > 0 {
                    remaining_outer_lambdas -= 1;
                    next_instruction <<= 1;
                } else {
                    break;
                }
            }
            if next_instruction == 1 {
                break;
            }
            result.push(next_instruction | 0b10000000);
        }

        result.extend_from_slice(&self.program);
        for application in self.applications.iter().rev() {
            result.extend_from_slice(application);
        }
        result.into_boxed_slice()
    }
}

const VARIABLE_NAMES: [&str; 16] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "l", "m", "n", "o", "p", "q"
];

fn show(program: &Program) -> String {
    let mut result = String::new();
    let mut lambda_index = 0;
    let mut flags = Flags::make();

    for instruction in &**program {
        let instruction = *instruction;
        if is_var(instruction) {
            if lambda_index <= instruction {
                let j = instruction - lambda_index;
                result.push_str(&format!("{}", j));
            } else {
                result.push_str(VARIABLE_NAMES[(lambda_index - 1 - instruction) as usize]);
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
        } else {
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

fn show_executor(executor: &ExecutionEnvironment) -> String {
    let mut result = show(&executor.program);
    if executor.outer_lambdas > 0 {
        result = format!("({})=>{}", executor.outer_lambdas-1, result);
    }
    for program in executor.applications.iter() {
        result.push_str("<");
        result.push_str(&show(program));
    }
    result
}

fn simplify_debug(program: Program) -> Program {
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

struct SimplifyEnv {
    path: Vec<(usize, ExecutionEnvironment)>
}

impl SimplifyEnv {
    fn make(program: Program) -> Self {
        SimplifyEnv {
            path: vec![(0, ExecutionEnvironment::make(program))]
        }
    }

    fn step(&mut self) -> bool {
        let len = self.path.len();
        let current = &mut self.path[len-1].1;

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
            self.path[len-2].1.applications[index] = current.to_program();
            if self.path[len-2].1.applications.len() > index + 1 {
                self.path.push((index + 1, ExecutionEnvironment::make(self.path[len-2].1.applications[index + 1].clone())));
                return true;
            }
        }
    }

    fn to_program(&self) -> Program {
        self.path[0].1.to_program()
    }
}

fn simplify(program: Program) -> Program {
    let mut simplifier = SimplifyEnv::make(program);
    
    while simplifier.step() {}

    simplifier.to_program()
}


// fn apply_1(prog: &str, arg: &str) -> String {
//     format!("83{prog}{arg}")
// }

// fn apply_2(prog: &str, arg1: &str, arg2: &str) -> String {
//     format!("87{prog}{arg1}{arg2}")
// }

const ZERO: &[u8; 2] = &[0x84, 0x00];
//const ONE: &[u8] = &[0x8C, 0x01, 0x00];
//const TWO: &[u8] = &[0x8C, 0x01, 0x83, 0x01, 0x00];

const ADD: &[u8] = &[0xF0, 0x03, 0x01, 0x87, 0x02, 0x01, 0x00];
const MUL: &[u8] = &[0x9C, 0x01, 0x83, 0xF0, 0x03, 0x01, 0x87, 0x02, 0x01, 0x00, 0x00, 0x84, 0x00];

fn number(n: u16) -> Program {
    if n == 0 { return Box::new(*ZERO); }
    let mut result = Vec::with_capacity(2*(n as usize)+1);
    result.push(0x8C);
    result.push(0x01);
    let mut index = n;
    while index > 1 {
        index -= 1;
        result.push(0x83);
        result.push(0x01);
    }
    result.push(0x00);
    result.into_boxed_slice()
}

fn apply2(prog: &[u8], arg1: &[u8], arg2: &[u8]) -> Program {
    let mut result = Vec::with_capacity(prog.len() + arg1.len() + arg2.len() + 1);
    result.push(0x87);
    result.extend_from_slice(prog);
    result.extend_from_slice(arg1);
    result.extend_from_slice(arg2);
    result.into_boxed_slice()
}

fn add(arg1: &[u8], arg2: &[u8]) -> Program {
    apply2(ADD, arg1, arg2)
}

fn mul(arg1: &[u8], arg2: &[u8]) -> Program {
    apply2(MUL, arg1, arg2)
}

fn main() {
    // let zero = "8400";
    // let one = "8C0100";
    // let two = "8C01830100";
    // let three = "8C018301830100";

    // let add = "F0030187020100";

    // let add1To1 = format!("87{add}{one}{one}");

    // // \x . x (\z. \a \b. a) (\a \b. b)
    // let eq0 = "8E0088018400";


    // let eq0_add1To1 = format!("83{eq0}{add1To1}");

    // //let prog = apply_1(eq0, one);
    // let prog = add1To1;

    // //let program = hex::decode("8500820000").unwrap().into_boxed_slice();
    // let program = hex::decode(prog).unwrap().into_boxed_slice();


    let program = add(&number(0), &number(10));

    let simplified = simplify_debug(program);
    println!("{:02X?}", &simplified);
    println!("Final result: {}", show(&simplified));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_add(d1: u16, d2: u16) {
        let program = add(&number(d1), &number(d2));

        let simplified = simplify(program);
        let compare = number(d1 + d2);

        assert!(simplified == compare)
    }

    fn test_mul(d1: u16, d2: u16) {
        let program = mul(&number(d1), &number(d2));

        let simplified = simplify(program);
        let compare = number(d1 * d2);

        assert!(simplified == compare)
    }

    fn test_add_debug(d1: u16, d2: u16) {
        let program = add(&number(d1), &number(d2));

        let simplified = simplify_debug(program);
        let compare = number(d1 + d2);

        assert!(simplified == compare)
    }


    #[test]
    fn test_some_adds() {
        test_add(0, 0);
        test_add(0, 1);
        test_add(0, 2);
        test_add(1, 0);
        test_add(2, 0);
        test_add(1, 1);
        test_add(10, 10);
        test_add(20, 20);
        test_add(25, 25);
        test_mul(100, 93);
    }

    #[test]
    fn test_some_add_debug() {
        test_add_debug(10, 20);
    }
}