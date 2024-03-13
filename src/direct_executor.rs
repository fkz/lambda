use std::rc::Rc;

#[derive(Clone, Copy)]

struct HeapReference {
    id: u32,
}

#[derive(Clone, Copy)]
struct Heap2Reference {
    id: u32,
}

impl Heap2Reference {
    pub fn first(&self) -> HeapReference {
        HeapReference { id: self.id }
    }

    pub fn second(&self) -> HeapReference {
        HeapReference { id: self.id + 1 }
    }
}

#[derive(Clone, Copy)]
enum Program {
    GlobalVar(u32),
    Var(u32),
    Lambda(HeapReference),
    App(Heap2Reference),
    Reference(HeapReference),
}

pub struct Executor {
    heap: Vec<Program>,
    stack: Vec<Program>,
    frames: Vec<u32>,
    lambdas: u64,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            stack: Vec::new(),
            frames: Vec::new(),
            lambdas: 0,
        }
    }

    fn add_heap(&mut self, p: Program) -> HeapReference {
        let result = HeapReference {
            id: self.heap.len() as u32,
        };
        self.heap.push(p);
        result
    }

    fn add_heap2(&mut self, p1: Program, p2: Program) -> Heap2Reference {
        let result = Heap2Reference {
            id: self.heap.len() as u32,
        };
        self.heap.push(p1);
        self.heap.push(p2);
        result
    }

    pub fn from_mem_rep(&mut self, program: &super::memory_representation::Program) -> Program {
        match program {
            super::memory_representation::Program::GlobalVar(id) => Program::GlobalVar(*id as u32),
            super::memory_representation::Program::Var(id) => Program::Var(*id as u32),
            super::memory_representation::Program::Lambda(p) => {
                let pp = self.from_mem_rep(p);
                Program::Lambda(self.add_heap(pp))
            }
            super::memory_representation::Program::App(p) => {
                let p0 = self.from_mem_rep(&p.0);
                let p1 = self.from_mem_rep(&p.1);
                Program::App(self.add_heap2(p0, p1))
            }
            super::memory_representation::Program::Reference(id) => {
                let pp = self.from_mem_rep(id);
                Program::Reference(self.add_heap(pp))
            }
        }
    }

    pub fn to_mem_rep(&self, p: Program) -> super::memory_representation::Program {
        match p {
            Program::GlobalVar(id) => super::memory_representation::Program::GlobalVar(id as u64),
            Program::Var(id) => super::memory_representation::Program::Var(id as u64),
            Program::Lambda(p) => super::memory_representation::Program::Lambda(Box::new(
                self.to_mem_rep(self.heap[p.id as usize]),
            )),
            Program::App(p) => super::memory_representation::Program::App(Box::new((
                self.to_mem_rep(self.heap[p.id as usize]),
                self.to_mem_rep(self.heap[p.id as usize + 1]),
            ))),
            Program::Reference(id) => super::memory_representation::Program::Reference(Rc::new(
                self.to_mem_rep(self.heap[id.id as usize]),
            )),
        }
    }

    pub fn initialize(&mut self, p: Program) {
        self.stack.clear();
        self.frames.clear();
        self.stack.push(p);
        self.frames.push(0);
        self.lambdas = 0;
    }

    fn program(&mut self) -> Program {
        assert!(self.frames.is_empty());
        assert_eq!(self.stack.len(), 1);
        if self.lambdas == 0 {
            self.stack[0]
        } else {
            let h = self.add_heap(self.stack[0]);
            let mut result = Program::Lambda(self.global_var_to_free(h, self.lambdas as u32));
            for _ in 1..self.lambdas {
                result = Program::Lambda(self.add_heap(result));
            }
            result
        }
    }

    fn from_heap(&self, p: HeapReference) -> Program {
        self.heap[p.id as usize]
    }

    fn from_heap2(&self, p: Heap2Reference) -> (Program, Program) {
        (self.heap[p.id as usize], self.heap[p.id as usize + 1])
    }

    fn replace(&mut self, p: Program, a: Program) -> Program {
        let mut to_replace = Vec::new();
        let aref = Program::Reference(self.add_heap(a));

        let result = {
            match p {
                Program::Lambda(p) => {
                    let r = self.add_heap(self.from_heap(p));
                    to_replace.push((r, 1));
                    Program::Lambda(r)
                }
                Program::App(p) => {
                    let (p1, p2) = self.from_heap2(p);
                    let r = self.add_heap2(p1, p2);
                    to_replace.push((r.first(), 0));
                    to_replace.push((r.second(), 0));
                    Program::App(r)
                }
                r @ Program::Reference(inner) => r,
                r @ Program::GlobalVar(_) => r,
                Program::Var(0) => aref,
                r @ Program::Var(_) => r,
            }
        };

        while let Some((next, var_index)) = to_replace.pop() {
            match self.from_heap(next) {
                Program::GlobalVar(_) => (),
                Program::Reference(p) => (),
                Program::Var(var) => {
                    if var == var_index {
                        self.heap[next.id as usize] = aref;
                    }
                }
                Program::Lambda(p) => {
                    let r = self.add_heap(self.from_heap(p));
                    to_replace.push((r, var_index + 1));
                    self.heap[next.id as usize] = Program::Lambda(r);
                }
                Program::App(p) => {
                    let (p1, p2) = self.from_heap2(p);
                    let r = self.add_heap2(p1, p2);
                    to_replace.push((r.first(), var_index));
                    to_replace.push((r.second(), var_index));
                    self.heap[next.id as usize] = Program::App(r);
                }
            }
        }

        result
    }

    fn global_var_to_free(&mut self, p: HeapReference, count: u32) -> HeapReference {
        let new_p = self.add_heap(self.from_heap(p));
        let mut to_replace = vec![(0, new_p)];

        while let Some((i, p)) = to_replace.pop() {
            match self.from_heap(p) {
                Program::GlobalVar(j) => self.heap[p.id as usize] = Program::Var(count - j - 1 + i),
                Program::Reference(p) => {
                    let r = self.add_heap(self.from_heap(p));
                    to_replace.push((i, r));
                    self.heap[p.id as usize] = Program::Reference(r);
                }
                Program::Lambda(p) => {
                    let r = self.add_heap(self.from_heap(p));
                    to_replace.push((i + 1, r));
                    self.heap[p.id as usize] = Program::Lambda(r);
                }
                Program::App(pp) => {
                    let (p1, p2) = self.from_heap2(pp);
                    let r = self.add_heap2(p1, p2);
                    to_replace.push((i, r.first()));
                    to_replace.push((i, r.second()));
                    self.heap[p.id as usize] = Program::App(r);
                }
                Program::Var(_) => (),
            }
        }

        new_p
    }

    fn global_var_intro(&mut self) -> bool {
        assert_eq!(self.stack.len(), 1);
        assert!(self.frames.is_empty());

        match self.stack[0] {
            Program::Lambda(h) => {
                let new_p =
                    self.replace(self.from_heap(h), Program::GlobalVar(self.lambdas as u32));
                self.lambdas += 1;
                self.stack[0] = new_p;
                self.frames.push(0);
                true
            }
            _ => false,
        }
    }

    pub fn step(&mut self) -> bool {
        let current_index = match self.frames.last() {
            None => return self.global_var_intro(),
            Some(i) => *i as usize,
        };
        let current = self.stack[current_index];
        match current {
            Program::App(p) => {
                let (f, app) = self.from_heap2(p);
                self.stack[current_index] = f;
                self.frames.push(self.stack.len() as u32);
                self.stack.push(app);
                true
            }
            Program::Reference(p) => {
                self.stack[current_index] = self.from_heap(p);
                true
            }
            Program::Lambda(p) => {
                if current_index < self.stack.len() - 1 {
                    let a = self.stack.pop().unwrap();
                    let new_p = self.replace(self.from_heap(p), a);
                    self.stack[current_index] = new_p;
                    true
                } else {
                    self.frames.pop().unwrap();
                    true
                }
            }
            Program::Var(_) => {
                panic!("Vars should not be possible, maybe there's an unused Var in the program?")
            }
            g @ Program::GlobalVar(_) => {
                let mut new_p = g;
                while current_index < self.stack.len() - 1 {
                    let a = self.stack.pop().unwrap();
                    new_p = Program::App(self.add_heap2(new_p, a));
                }
                self.stack[current_index] = new_p;
                self.frames.pop().unwrap();
                true
            }
        }
    }

    pub fn show(&self) -> String {
        let mut result = String::new();
        if self.lambdas > 0 {
            result += format!("({})=>", self.lambdas).as_str();
        }
        let mut frame_iter = self.frames.iter();
        let mut next_frame = frame_iter.next();
        for (i, p) in self.stack.iter().enumerate() {
            if Some(&(i as u32)) == next_frame {
                result += "|";
                next_frame = frame_iter.next();
            } else {
                result += "<";
            }
            result += super::program::show(&self.to_mem_rep(*p).to_opcode()).as_str();
        }
        result
    }
}

pub fn from_rep(program: super::memory_representation::Program) -> Executor {
    let mut result = Executor::new();
    let p = result.from_mem_rep(&program);
    result.initialize(p);
    result
}

pub fn to_rep(mut executor: Executor) -> super::memory_representation::Program {
    let p = executor.program();
    executor.to_mem_rep(p)
}
