use crate::program::Program;

pub const EXIT_SUCCESFULLY: &[u8] = &[0x9C, 0x00, 0x88, 0x01, 0x82, 0x00];
pub const EXIT_AFTER_START: &[u8] = &[
    0x86, 0x00, 0x9C, 0x01, 0x8E, 0x00, 0x88, 0x01, 0x82, 0x00, 0x82, 0x00,
];
pub const EXIT_AFTER_READ: &[u8] = &[0x9C, 0x00, 0x88, 0x02, 0x87, 0x00, 0x88, 0x01, 0x82, 0x00];

pub const REPEAT: &[u8] = &[
    0xE8, 0x86, 0x01, 0x83, 0x00, 0x00, 0x86, 0x01, 0x83, 0x00, 0x00, 0x8E, 0x01, 0x03, 0x00,
];

pub fn repeat_print_h() -> Program {
    let print_h = super::simple_env::Request::Print('h' as u8).to_program();
    super::example_programs::apply1(REPEAT, &print_h)
}

pub const PRINT_FOR_EACH_READ: &[u8] = &[
    0x8C, 0x01, 0x9C, 0x02, 0xF8, 0xBF, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84,
    0x00, 0x84, 0x00, 0x84, 0x01, 0x84, 0x00, 0x87, 0x02, 0x88, 0x02, 0x00,
];

pub const PRINT_READ: &[u8] = &[
    0x8C, 0x01, 0x9C, 0x02, 0x87, 0x01, 0xF8, 0xBF, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x00, 0x84,
    0x00, 0x84, 0x00, 0x84, 0x00, 0x84, 0x01, 0x84, 0x00, 0xC0, 0xE0, 0xFF, 0x83, 0x00, 0x0A, 0x09,
    0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x87, 0x02, 0x88, 0x02, 0x00,
];

pub fn hello_world() -> Program {
    let mut hello_world: Vec<u8> = Vec::new();
    hello_world.push(0x84);
    for char in "Hello World\n".chars().into_iter() {
        let byte = char as u8;
        let as_prog: Box<[u8]> = crate::simple_env::Request::Print(byte).to_program();
        hello_world.push(0x87);
        hello_world.push(0x00);
        hello_world.extend_from_slice(&as_prog);
    }
    hello_world.push(0x87);
    hello_world.push(0x00);
    hello_world.push(0x88);
    hello_world.push(0x01);
    hello_world.push(0x82);
    hello_world.push(0x00);

    hello_world.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use crate::interact::interact;
    use crate::simple_env::MockEnv;
    use crate::simple_env::Request::*;
    use crate::simple_env::RequestOrResponse::{Request, Response};
    use crate::simple_env::Response::*;

    #[test]
    fn test_hello_world() {
        let mut env = MockEnv::make(&[
            Response(Start),
            Request(Print('H' as u8)),
            Request(Print('e' as u8)),
            Request(Print('l' as u8)),
            Request(Print('l' as u8)),
            Request(Print('o' as u8)),
            Request(Print(' ' as u8)),
            Request(Print('W' as u8)),
            Request(Print('o' as u8)),
            Request(Print('r' as u8)),
            Request(Print('l' as u8)),
            Request(Print('d' as u8)),
            Request(Print('\n' as u8)),
            Request(Exit),
        ]);

        interact(&mut env, &super::hello_world(), false, false);

        assert_eq!(env.failed(), None)
    }
}
