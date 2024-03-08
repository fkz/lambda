use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    rc::Rc,
};

pub trait HasBox2 {
    type Box2;
}

#[derive(Copy, Clone)]
pub enum Program<Boxed: HasBox2> {
    Lambda(Boxed),
    App(Boxed::Box2),
    Var(u32),
    GlobalVar(u32),
    Reference(Boxed),
    InternalReplaced(u32),
}

impl<B: HasBox2> Program<B> {
    fn map<C: HasBox2, F: FnOnce(&B) -> C, F2: FnOnce(&B::Box2) -> C::Box2>(
        &self,
        f: F,
        f2: F2,
    ) -> Program<C> {
        match self {
            Program::Lambda(b) => Program::Lambda(f(b)),
            Program::App(b) => Program::App(f2(b)),
            Program::Var(u) => Program::Var(*u),
            Program::GlobalVar(u) => Program::GlobalVar(*u),
            Program::Reference(b) => Program::Reference(f(b)),
            Program::InternalReplaced(x) => Program::InternalReplaced(*x),
        }
    }

    fn box1(&self) -> Option<&B> {
        match self {
            Program::Lambda(b) => Some(b),
            Program::Reference(b) => Some(b),
            _ => None,
        }
    }
    fn box2(&self) -> Option<&B::Box2> {
        match self {
            Program::App(b) => Some(b),
            _ => None,
        }
    }

    fn set_box1(&mut self, b: B) {
        match self {
            Program::Lambda(bb) => *bb = b,
            Program::Reference(bb) => *bb = b,
            _ => (),
        }
    }
    fn set_box2(&mut self, b: B::Box2) {
        match self {
            Program::App(bb) => *bb = b,
            _ => (),
        }
    }
}

pub trait Allocator: Sized {
    type LongBox;
    type ShortBox<'a>: HasBox2 + Clone + Copy
    where
        Self: 'a;
    type Ref;

    fn new<'a>(&'a self, value: &Program<Self::ShortBox<'a>>) -> Self::ShortBox<'a>;
    fn new2<'a>(
        &'a self,
        value: &Program<Self::ShortBox<'a>>,
        value2: &Program<Self::ShortBox<'a>>,
    ) -> <Self::ShortBox<'a> as HasBox2>::Box2;
    fn replace<'a>(&'a self, value: Self::ShortBox<'a>, replacement: Program<Self::ShortBox<'a>>);
    fn pair<'a>(
        &'a self,
        v: <Self::ShortBox<'a> as HasBox2>::Box2,
    ) -> (Self::ShortBox<'a>, Self::ShortBox<'a>);

    fn to_long<'a>(&'a self, value: &Self::ShortBox<'a>) -> Self::LongBox;
    fn to_short<'a>(&'a self, value: &Self::LongBox) -> Self::ShortBox<'a>;
    fn deref_short<'a>(&'a self, value: &Self::ShortBox<'a>) -> Program<Self::ShortBox<'a>>;

    fn remove_long(long: &mut Self::LongBox);

    fn collect_garbage(&mut self, print_info: bool);
}

pub mod alloc_impl {
    use std::{
        cell::{Cell, Ref, RefCell},
        marker::PhantomData,
        mem,
        rc::Rc,
    };

    pub struct LongBox<A: super::Allocator<LongBox = LongBox<A>>> {
        value: u32,
        a: A::Ref,
    }

    impl<A: super::Allocator<LongBox = LongBox<A>>> Drop for LongBox<A> {
        fn drop(&mut self) {
            A::remove_long(self);
        }
    }

    #[derive(Copy, Clone)]
    struct InternalBox {
        value: u32,
    }

    impl super::HasBox2 for InternalBox {
        type Box2 = InternalBox;
    }

    #[derive(Clone, Copy)]
    pub struct ShortBox<'a> {
        value: InternalBox,
        phantom: PhantomData<&'a ()>,
    }

    impl<'a> super::HasBox2 for ShortBox<'a> {
        type Box2 = ShortBox<'a>;
    }

    pub struct Allocator {
        memory: RefCell<Vec<super::Program<InternalBox>>>,
        other: RefCell<Vec<super::Program<InternalBox>>>,
        roots: RefCell<Vec<InternalBox>>,
        free_roots: RefCell<Vec<u32>>,
    }

    impl Allocator {
        fn copy_over(&mut self, b: InternalBox) -> InternalBox {
            let value = &mut self.memory.get_mut()[b.value as usize];
            let other = self.other.get_mut();
            if let super::Program::InternalReplaced(v) = *value {
                InternalBox { value: v }
            } else {
                other.push(*value);
                *value = super::Program::InternalReplaced(other.len() as u32 - 1);
                InternalBox {
                    value: other.len() as u32 - 1,
                }
            }
        }
    }

    impl super::Allocator for Allocator {
        type LongBox = LongBox<Self>;
        type ShortBox<'a> = ShortBox<'a>;
        type Ref = ();

        fn new<'a>(&'a self, value: &super::Program<Self::ShortBox<'a>>) -> Self::ShortBox<'a> {
            let new_value: super::Program<InternalBox> = value.map(|x| x.value, |x| x.value);
            let mut b = self.memory.borrow_mut();
            b.push(new_value);
            ShortBox {
                value: InternalBox {
                    value: b.len() as u32 - 1,
                },
                phantom: PhantomData,
            }
        }

        fn new2<'a>(
            &'a self,
            value: &super::Program<Self::ShortBox<'a>>,
            value2: &super::Program<Self::ShortBox<'a>>,
        ) -> <Self::ShortBox<'a> as super::HasBox2>::Box2 {
            let new_value: super::Program<InternalBox> = value.map(|x| x.value, |x| x.value);
            let mut b = self.memory.borrow_mut();
            b.push(new_value);
            let new_value2: super::Program<InternalBox> = value2.map(|x| x.value, |x| x.value);
            b.push(new_value2);
            ShortBox {
                value: InternalBox {
                    value: b.len() as u32 - 2,
                },
                phantom: PhantomData,
            }
        }

        fn replace<'a>(
            &'a self,
            value: Self::ShortBox<'a>,
            replacement: super::Program<Self::ShortBox<'a>>,
        ) {
            let mut b = self.memory.borrow_mut();
            b[value.value.value as usize] = replacement.map(|x| x.value, |x| x.value);
        }

        fn pair<'a>(
            &'a self,
            v: <Self::ShortBox<'a> as super::HasBox2>::Box2,
        ) -> (Self::ShortBox<'a>, Self::ShortBox<'a>) {
            (
                ShortBox {
                    value: InternalBox {
                        value: v.value.value,
                    },
                    phantom: PhantomData,
                },
                ShortBox {
                    value: InternalBox {
                        value: v.value.value + 1,
                    },
                    phantom: PhantomData,
                },
            )
        }

        fn to_long<'a>(&self, value: &Self::ShortBox<'a>) -> Self::LongBox {
            let mut roots = self.roots.borrow_mut();
            if let Some(next) = self.free_roots.borrow_mut().pop() {
                roots[next as usize] = value.value;
                LongBox { value: next, a: () }
            } else {
                let root_id = roots.len();
                roots.push(value.value);
                LongBox {
                    value: root_id as u32,
                    a: (),
                }
            }
        }

        fn to_short<'a>(&'a self, value: &Self::LongBox) -> Self::ShortBox<'a> {
            ShortBox {
                value: self.roots.borrow_mut()[value.value as usize],
                phantom: PhantomData,
            }
        }

        fn remove_long(long: &mut Self::LongBox) {
            let allocator = unsafe { &ALLOCATOR };
            allocator.roots.borrow_mut()[long.value as usize] = InternalBox {
                value: u32::max_value(),
            };
            allocator.free_roots.borrow_mut().push(long.value);
        }

        fn deref_short<'a>(
            &'a self,
            value: &Self::ShortBox<'a>,
        ) -> super::Program<Self::ShortBox<'a>> {
            self.memory.borrow()[value.value.value as usize].map(
                |x| ShortBox {
                    value: *x,
                    phantom: PhantomData,
                },
                |x| ShortBox {
                    value: *x,
                    phantom: PhantomData,
                },
            )
        }

        fn collect_garbage(&mut self, print_info: bool) {
            let mut other = self.other.get_mut();
            other.clear();
            other.reserve_exact(4096);

            let mut finished_work = 0;
            // Copy over roots
            for root in self.roots.get_mut().iter_mut() {
                if root.value != u32::max_value() {
                    // TODO Use copy_over instead of inlining here
                    let new_ref = {
                        let value = &mut self.memory.get_mut()[root.value as usize];
                        let other = self.other.get_mut();
                        if let super::Program::InternalReplaced(v) = *value {
                            InternalBox { value: v }
                        } else {
                            other.push(*value);
                            *value = super::Program::InternalReplaced(other.len() as u32 - 1);
                            InternalBox {
                                value: other.len() as u32 - 1,
                            }
                        }
                    };

                    *root = new_ref;
                }
            }

            let root_count = self.other.get_mut().len();

            while finished_work < self.other.get_mut().len() {
                let next = self.other.get_mut()[finished_work];

                if let super::Program::InternalReplaced(v) = next {
                    panic!("This should never happen");
                } else {
                    if let Some(b) = next.box1() {
                        let new_box = self.copy_over(*b);
                        self.other.get_mut()[finished_work].set_box1(new_box);
                    } else if let Some(b) = next.box2() {
                        let new_box = self.copy_over(*b);
                        let new_box2 = self.copy_over(InternalBox { value: b.value + 1 });
                        self.other.get_mut()[finished_work].set_box2(new_box);
                        assert!(new_box.value == new_box2.value - 1)
                    }
                }

                finished_work += 1;
            }

            if print_info {
                println!(
                    "After garbage collection, keeping {} from {} boxes; out of {} roots",
                    self.other.get_mut().len(),
                    self.memory.get_mut().len(),
                    root_count
                );
            }
            std::mem::swap(self.memory.get_mut(), self.other.get_mut());
        }
    }

    static mut ALLOCATOR: Allocator = Allocator {
        memory: RefCell::new(Vec::new()),
        other: RefCell::new(Vec::new()),
        roots: RefCell::new(Vec::new()),
        free_roots: RefCell::new(Vec::new()),
    };
    static mut CAPTURED: bool = false;

    pub fn get_allocator() -> &'static mut Allocator {
        unsafe {
            if CAPTURED {
                panic!("Allocator can only be created once");
            } else {
                CAPTURED = true;
                let _ = ALLOCATOR.memory.get_mut().try_reserve_exact(4096);
                &mut ALLOCATOR
            }
        }
    }
}

#[test]
fn test() {
    let alloc = alloc_impl::get_allocator();

    let lambda = Program::Lambda(alloc.new(&Program::Var(0)));

    {
        let c = alloc.new(&lambda);
        let a = alloc.new(&lambda);
        let b = alloc.to_long(&a);
    }

    alloc.collect_garbage(true);

    //let f = Program::Lambda(alloc.to_short(&b));
    //alloc.new(&f);

    assert_eq!(1, 2);
}

pub struct Executor<A: Allocator> {
    current: A::LongBox,
    app_stack: Vec<A::LongBox>,
    previous: Vec<(A::LongBox, Vec<A::LongBox>)>,
    lambdas: u64,
}

impl<A: Allocator> Executor<A> {
    pub fn new(program: A::LongBox) -> Self {
        Self {
            current: program,
            app_stack: Vec::new(),
            previous: Vec::new(),
            lambdas: 0,
        }
    }

    pub fn to_program<'a>(self, a: &'a A) -> A::ShortBox<'a> {
        assert_eq!(self.previous.len(), 0);

        let mut app_stack = self.app_stack;
        let mut result = a.deref_short(&a.to_short(&self.current));
        while let Some(next) = app_stack.pop() {
            let next = a.to_short(&next);
            result = Program::App(a.new2(&result, &a.deref_short(&next)));
        }

        let mut result = Executor::replace_glob(a, a.new(&result), self.lambdas);
        for _ in 0..self.lambdas {
            result = a.new(&Program::Lambda(result));
        }
        result
    }

    fn replace<'a>(a: &'a A, f: A::ShortBox<'a>, app: A::ShortBox<'a>) -> A::ShortBox<'a> {
        let mut to_do = Vec::new();
        to_do.push((f, 0));

        while let Some((current, depth)) = to_do.pop() {
            match a.deref_short(&current) {
                Program::Lambda(p) => {
                    to_do.push((p, depth + 1));
                }
                Program::App(f) => {
                    let (f, app) = a.pair(f);
                    to_do.push((f, depth));
                    to_do.push((app, depth));
                }
                Program::Var(v) => {
                    if v == depth {
                        a.replace(current, Program::Reference(app));
                    }
                }
                Program::GlobalVar(_) => {
                    // Do nothing
                }
                Program::Reference(_) => {
                    // Do nothing
                }
                Program::InternalReplaced(_) => {
                    panic!("This should never happen!")
                }
            }
        }

        f
    }

    fn replace_glob<'a>(a: &'a A, f: A::ShortBox<'a>, glob_count: u64) -> A::ShortBox<'a> {
        let mut f = f;

        let mut to_do = Vec::new();
        to_do.push((f, 0));

        while let Some((current, depth)) = to_do.pop() {
            match a.deref_short(&current) {
                Program::InternalReplaced(_) => panic!("This should never happen!"),
                Program::Lambda(p) => {
                    to_do.push((p, depth + 1));
                }
                Program::App(f) => {
                    let (f, app) = a.pair(f);
                    to_do.push((f, depth));
                    to_do.push((app, depth));
                }
                Program::Var(v) => {
                    if v >= depth {
                        panic!("unused var {} >= {}", v, depth);
                    }
                }
                Program::GlobalVar(q) => {
                    a.replace(current, Program::Var(glob_count as u32 - q - 1));
                }
                Program::Reference(q) => {
                    to_do.push((q, depth));
                }
            }
        }

        f
    }

    fn app<'a>(
        a: &'a A,
        current: Program<A::ShortBox<'a>>,
        mut args: Vec<Program<A::ShortBox<'a>>>,
    ) -> Program<A::ShortBox<'a>> {
        if let Some(last_app) = args.pop() {
            let p = Executor::app(a, current, args);
            Program::App(a.new2(&p, &last_app))
        } else {
            current
        }
    }

    pub fn step(mut self, a: &A) -> (Self, bool) {
        match a.deref_short(&a.to_short(&self.current)) {
            Program::InternalReplaced(_) => panic!("This should never happen!"),
            Program::Lambda(p) => {
                if let Some(app) = self.app_stack.pop() {
                    self.current = a.to_long(&Executor::replace(a, p, a.to_short(&app)));
                    (self, true)
                } else {
                    if let Some((previous_current, previous_app)) = self.previous.pop() {
                        self.app_stack = previous_app;
                        self.app_stack.push(a.to_long(&a.new(&Program::Lambda(p))));
                        self.current = previous_current;
                        (self, true)
                    } else {
                        self.current = a.to_long(&Executor::replace(
                            a,
                            p,
                            a.new(&Program::GlobalVar(self.lambdas as u32)),
                        ));
                        self.lambdas += 1;
                        (self, true)
                    }
                }
            }
            Program::App(f) => {
                let (f, app) = a.pair(f);
                self.previous.push((a.to_long(&f), self.app_stack));
                self.app_stack = Vec::new();
                self.current = a.to_long(&app);
                (self, true)
            }
            Program::Var(v) => {
                panic!("Unbound var {}", v);
            }
            Program::GlobalVar(_) => {
                if let Some((previous_f, previous_app)) = self.previous.pop() {
                    let mut current = self.current;
                    if self.app_stack.len() > 0 {
                        current = a.to_long(
                            &a.new(&Executor::app(
                                a,
                                a.deref_short(&a.to_short(&current)),
                                self.app_stack
                                    .into_iter()
                                    .map(|x| a.deref_short(&a.to_short(&x)))
                                    .collect(),
                            )),
                        );
                    }
                    self.app_stack = previous_app;
                    self.app_stack.push(current);
                    self.current = previous_f;
                    (self, true)
                } else {
                    (self, false)
                }
            }
            Program::Reference(p) => {
                self.current = a.to_long(&p);
                (self, true)
            }
        }
    }

    /*pub fn show(&self) -> String {
        let mut result = super::show(&self.current.clone().to_opcode());
        for app in self.app_stack.iter().rev() {
            result.push_str("<");
            result.push_str(&super::show(&app.clone().to_opcode()));
        }
        for (c, c_app) in self.previous.iter() {
            result.push_str("\n");
            result.push_str(&super::show(&c.clone().to_opcode()));
            for app in c_app.iter().rev() {
                result.push_str("<");
                result.push_str(&super::show(&app.clone().to_opcode()));
            }
        }
        result
    }*/
}

pub fn to_mem_rep<'a, A: Allocator>(
    a: &'a A,
    p: Program<A::ShortBox<'a>>,
) -> super::memory_representation::Program {
    match p {
        Program::Lambda(b) => super::memory_representation::Program::Lambda(Box::new(to_mem_rep(
            a,
            a.deref_short(&b),
        ))),
        Program::App(b) => {
            let (b1, b2) = a.pair(b);
            super::memory_representation::Program::App(Box::new((
                to_mem_rep(a, a.deref_short(&b1)),
                to_mem_rep(a, a.deref_short(&b2)),
            )))
        }
        Program::Var(u) => super::memory_representation::Program::Var(u as u64),
        Program::GlobalVar(u) => super::memory_representation::Program::GlobalVar(u as u64),
        Program::Reference(b) => super::memory_representation::Program::Reference(Rc::new(
            to_mem_rep(a, a.deref_short(&b)),
        )),
        Program::InternalReplaced(x) => panic!("Internal error"),
    }
}

pub fn from_mem_rep<'a, A: Allocator>(
    a: &'a A,
    p: &super::memory_representation::Program,
) -> Program<A::ShortBox<'a>> {
    match p {
        super::memory_representation::Program::Lambda(b) => {
            Program::Lambda(a.new(&from_mem_rep(a, &**b)))
        }
        super::memory_representation::Program::App(b) => {
            let b = a.new2(&from_mem_rep(a, &b.0), &from_mem_rep(a, &b.1));
            Program::App(b)
        }
        super::memory_representation::Program::Var(u) => Program::Var(*u as u32),
        super::memory_representation::Program::GlobalVar(u) => Program::GlobalVar(*u as u32),
        super::memory_representation::Program::Reference(b) => {
            Program::Reference(a.new(&from_mem_rep(a, b)))
        }
    }
}
