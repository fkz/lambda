use crate::program::Program;

const ZERO: &[u8; 2] = &[0x84, 0x00];
const ONE: &[u8] = &[0x8C, 0x01, 0x00];
const TWO: &[u8] = &[0x8C, 0x01, 0x83, 0x01, 0x00];

const ADD: &[u8] = &[0xF0, 0x03, 0x01, 0x87, 0x02, 0x01, 0x00];
const MUL: &[u8] = &[0x9C, 0x01, 0x83, 0xF0, 0x03, 0x01, 0x87, 0x02, 0x01, 0x00, 0x00, 0x84, 0x00];
const PRED: &[u8] = &[0xF8, 0x02, 0x8C, 0x00, 0x83, 0x01, 0x03, 0x82, 0x01, 0x82, 0x00];
const SUB: &[u8] = &[0x9C, 0x00, 0xF8, 0x02, 0x8C, 0x00, 0x83, 0x01, 0x03, 0x82, 0x01, 0x82, 0x00, 0x01];

const TUPLE: &[u8] = &[0xB8, 0x00, 0x02, 0x01];

const TRUE: &[u8] = &[0x84, 0x01];
const FALSE: &[u8] = &[0x84, 0x00];

const BYTE0: &[u8] = &[0xFE, 0x8F, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00];

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

fn apply1(prog: &[u8], arg: &[u8]) -> Program {
  let mut result = Vec::with_capacity(prog.len() + arg.len() + 1);
  result.push(0x83);
  result.extend_from_slice(prog);
  result.extend_from_slice(arg);
  result.into_boxed_slice()
}

fn add(arg1: &[u8], arg2: &[u8]) -> Program {
  apply2(ADD, arg1, arg2)
}

fn pred(arg: &[u8]) -> Program {
  apply1(PRED, arg)
}

fn mul(arg1: &[u8], arg2: &[u8]) -> Program {
  apply2(MUL, arg1, arg2)
}

fn sub(arg1: &[u8], arg2: &[u8]) -> Program {
  apply2(SUB, arg1, arg2)
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::program::{simplify, simplify_debug};


    fn test_op(op: &'static [u8], f: Box<dyn Fn(u16, u16) -> u16>) -> Box<dyn Fn(u16, u16)> {
      Box::new(move | arg1, arg2 | {
        let program = apply2(op, &number(arg1), &number(arg2));

        assert_eq!(crate::program::verify(&program), Ok(0));

        let simplified = simplify(program);
        let compare = number(f(arg1, arg2));

        assert!(simplified == compare)
      })
    }

    fn test_add_debug(d1: u16, d2: u16) {
        let program = add(&number(d1), &number(d2));

        let simplified = simplify_debug(program);
        let compare = number(d1 + d2);

        assert!(simplified == compare)
    }


    #[test]
    fn test_some_adds() {
        let test_add = test_op(ADD, Box::new(|x, y| x + y));
        let test_mul = test_op(MUL, Box::new(|x, y| x * y));
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