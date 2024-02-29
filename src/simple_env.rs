use crate::program::Program;

#[derive(Copy, Clone, Debug)]
pub enum Response {
    Start,
    ReadByte(u8),
}

impl crate::interact::Response for Response {
    fn to_program(self) -> Program {
        match self {
            Response::Start => vec![0x84, 0x01].into_boxed_slice(),
            Response::ReadByte(byte) => {
                let mut result = Vec::new();
                result.push(0xFC);
                result.push(0x9F);
                result.push(0x00);

                for i in 0..8 {
                    let bit = byte & (1 << i);
                    result.push(0x84);
                    if bit == 0 {
                        result.push(0x00);
                    } else {
                        result.push(0x01);
                    }
                }

                result.into_boxed_slice()
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Request {
    Print(u8),
    Exit,
    Read,
}

impl crate::interact::Request for Request {
    fn from_program(program: &Program) -> Option<Request> {
        match program[0] {
            0x88 => match program[1] {
                0x01 => Some(Request::Exit),
                0x02 => Some(Request::Read),
                _ => None,
            },
            0xF8 => match program[1] {
                0xBF => match program[2] {
                    0x00 => {
                        let mut byte = 0;
                        for bit in 0..8 {
                            let index = bit * 2 + 3;
                            if program[index] != 0x84 || program[index + 1] >= 2 {
                                return None;
                            }
                            byte |= (program[index + 1] & 1) << bit;
                        }
                        Some(Request::Print(byte))
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}

impl Request {
    pub fn to_program(&self) -> Program {
        match self {
            Request::Print(b) => {
                let mut result = Vec::new();
                result.push(0xF8);
                result.push(0xBF);
                result.push(0x00);
                for index in 0..8 {
                    result.push(0x84);
                    if b & (1 << index) == 0 {
                        result.push(0x00);
                    } else {
                        result.push(0x01);
                    }
                }
                result.into_boxed_slice()
            }
            Request::Exit => [0x88, 0x01].to_vec().into_boxed_slice(),
            Request::Read => [0x88, 0x02].to_vec().into_boxed_slice(),
        }
    }
}

pub struct Env {
    started: bool,
    finished: bool,
    read: Option<u8>,
}

impl crate::interact::Environment<Request, Response> for Env {
    fn finished(&self) -> bool {
        self.finished
    }

    fn make() -> Self {
        Env {
            started: false,
            finished: false,
            read: None,
        }
    }

    fn request(&mut self, request: Request) {
        //println!("req: {:?}", request);
        match request {
            Request::Exit => {
                println!("Succesfully exiting");
                self.finished = true;
            }
            Request::Print(byte) => {
                print!("{}", char::from_u32(byte as u32).unwrap());
            }
            Request::Read => {
                let mut buffer: [u8; 1] = [0x00];
                std::io::Read::read(&mut std::io::stdin(), &mut buffer).unwrap();
                self.read = Some(buffer[0]);
            }
        }
    }

    fn next_response(&mut self) -> Option<Response> {
        if !self.started {
            self.started = true;
            Some(Response::Start)
        } else if let Some(byte) = self.read {
            //println!("response: ReadByte {}", byte);
            self.read = None;
            Some(Response::ReadByte(byte))
        } else {
            None
        }
    }

    fn panic(&mut self, panic_info: crate::interact::PanicInfo) {
        self.finished = true;
        println!("panic {:?}", panic_info);
    }
}

#[derive(Clone)]
pub enum RequestOrResponse {
    Request(Request),
    Response(Response),
}

pub struct MockEnv {
    values: Vec<RequestOrResponse>,
    failed: Option<String>,
}

impl crate::interact::Environment<Request, Response> for MockEnv {
    fn finished(&self) -> bool {
        self.values.is_empty() || self.failed.is_some()
    }

    fn make() -> Self {
        MockEnv {
            values: Vec::new(),
            failed: None,
        }
    }

    fn next_response(&mut self) -> Option<Response> {
        match self.values.last() {
            None => None,
            Some(RequestOrResponse::Response(_)) => {
                if let Some(RequestOrResponse::Response(resp)) = self.values.pop() {
                    Some(resp)
                } else {
                    None
                }
            }
            Some(_) => None,
        }
    }

    fn request(&mut self, request: Request) {
        match self.values.pop() {
            None => self.failed = Some(format!("Request {:?} received at end", request)),
            Some(RequestOrResponse::Response(resp)) => {
                self.failed = Some(format!(
                    "Request {:?} received when response {:?} was expected",
                    request, resp
                ))
            }
            Some(RequestOrResponse::Request(req)) if req == request => (),
            Some(RequestOrResponse::Request(req)) => {
                self.failed = Some(format!(
                    "Request {:?} expected, but got Request {:?}",
                    req, request
                ))
            }
        }
    }

    fn panic(&mut self, panic_info: crate::interact::PanicInfo) {
        self.failed = Some(format!("Panicked with {:?}", panic_info));
    }
}

impl MockEnv {
    pub fn make(list: &[RequestOrResponse]) -> Self {
        let mut values = list.to_vec();
        values.reverse();
        Self {
            values: values,
            failed: None,
        }
    }

    pub fn failed(&self) -> Option<String> {
        self.failed.clone()
    }
}
