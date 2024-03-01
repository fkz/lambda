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
        let expected_number = {
            if p.len() == 2 {
                0
            } else if p.len() % 2 == 1 {
                p.len() / 2
            } else {
                return None;
            }
        };

        if expected_number == 0 && p != [0x84, 0x00] {
            return None;
        };
        if expected_number > 0 && p[0..2] != [0x8C, 0x01] {
            return None;
        };
        if expected_number > 0 && p.last() != Some(&0x00) {
            return None;
        }

        for i in 2..(p.len() - 1) {
            if (i % 2 == 0 && p[i] != 0x83) || (i % 2 == 1 && p[i] != 0x01) {
                return None;
            }
        }

        Some(expected_number.to_string())
    }
}
