use crate::compare::Inst;

use super::program::Program;

pub trait Pretty {
    fn program_to_string(&self, p: &[u8]) -> Option<String>;
    fn string_to_program(&self, s: &str) -> Option<Program>;
    fn name(&self) -> String;
}

pub struct Number;

impl Pretty for Number {
    fn name(&self) -> String {
        "number".to_string()
    }

    fn string_to_program(&self, s: &str) -> Option<Program> {
        let number: u32 = s.parse().ok()?;
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

    fn program_to_string(&self, p: &[u8]) -> Option<String> {
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
                Inst::Var(0) => return Some(result.to_string()),
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
