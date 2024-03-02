use crate::compare::Inst;

use super::program::Program;

pub trait Pretty<T> {
    fn program_to_string(&self, p: &[u8]) -> Option<T>;
    fn string_to_program(&self, s: T) -> Option<Program>;
    fn name(&self) -> String;
}

struct Number;
struct Nothing;

pub fn number() -> Box<dyn Pretty<String>> {
    Box::new(Number)
}

pub fn nothing() -> Box<dyn Pretty<String>> {
    Box::new(Nothing)
}

pub fn number_u32() -> Box<dyn Pretty<u32>> {
    Box::new(Number)
}

impl Pretty<String> for Nothing {
    fn name(&self) -> String {
        "nothing".to_string()
    }

    fn string_to_program(&self, _s: String) -> Option<Program> {
        None
    }

    fn program_to_string(&self, p: &[u8]) -> Option<String> {
        Some(crate::program::show(p))
    }
}

impl Pretty<u32> for Number {
    fn name(&self) -> String {
        "number".to_string()
    }

    fn string_to_program(&self, number: u32) -> Option<Program> {
        if number == 0 {
            return Some(Box::new([0x84, 0x00]));
        }

        let mut result = Vec::with_capacity(2 * (number as usize) + 1);
        result.push(0x8C);
        result.push(0x01);
        let mut index = number;
        while index > 1 {
            index -= 1;
            result.push(0x83);
            result.push(0x01);
        }
        result.push(0x00);
        Some(result.into_boxed_slice())
    }

    fn program_to_string(&self, p: &[u8]) -> Option<u32> {
        let mut result = 0;

        let mut iter = super::compare::prog_iter(p);

        if iter.next()? != Inst::Lambda {
            return None;
        };
        if iter.next()? != Inst::Lambda {
            return None;
        };

        loop {
            match iter.next()? {
                Inst::Var(0) => return Some(result),
                Inst::Apply => {
                    if iter.next()? != Inst::Var(1) {
                        return None;
                    };
                }
                _ => return None,
            }
            result += 1;
        }
    }
}

impl Pretty<String> for Number {
    fn name(&self) -> String {
        "number".to_string()
    }

    fn string_to_program(&self, s: String) -> Option<Program> {
        let number: u32 = s.parse().ok()?;
        Pretty::<u32>::string_to_program(self, number)
    }

    fn program_to_string(&self, p: &[u8]) -> Option<String> {
        let result = Pretty::<u32>::program_to_string(self, p);
        Some(result?.to_string())
    }
}
