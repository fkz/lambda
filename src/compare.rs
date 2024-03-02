#[derive(PartialEq)]
pub enum Inst {
    Apply,
    Lambda,
    Var(u64),
}

pub struct ProgIterator<'a> {
    program: &'a [u8],
    flags: u8,
}

impl<'a> Iterator for ProgIterator<'a> {
    type Item = Inst;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flags == 1 {
            if self.program.is_empty() {
                None
            } else if self.program[0] & 0b10000000 == 0 {
                let mut var: u64 = 0;
                while self.program[0] & 0b01000000 == 0b01000000 {
                    var <<= 6;
                    var |= (self.program[0] & 0b00111111) as u64;
                    self.program = &self.program[1..];
                }
                var <<= 6;
                var |= (self.program[0] & 0b00111111) as u64;
                self.program = &self.program[1..];
                Some(Inst::Var(var))
            } else {
                self.flags = self.program[0] & 0b01111111;
                self.program = &self.program[1..];
                self.next()
            }
        } else {
            let result = if self.flags & 1 == 0 {
                Inst::Lambda
            } else {
                Inst::Apply
            };
            self.flags >>= 1;
            Some(result)
        }
    }
}

pub fn prog_iter(a: &[u8]) -> ProgIterator {
    ProgIterator {
        flags: 1,
        program: a,
    }
}
