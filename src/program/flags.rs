pub struct Flags {
    flags: u128,
    more_flags: Vec<u64>,
    shrink: u8,
}

impl Flags {
    pub fn make() -> Self {
        Flags {
            flags: 2,
            more_flags: Vec::new(),
            shrink: 126,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.flags == 0 && self.more_flags.is_empty()
    }

    fn extend(&mut self) {
        if self.shrink == 0 {
            let value = (self.flags >> 64) as u64;
            self.flags &= (1 << 64) - 1;
            self.shrink = 64;
            self.more_flags.push(value);
        }
        self.shrink -= 2;
        self.flags <<= 2;
    }

    fn shrink(&mut self) {
        self.flags >>= 2;
        self.shrink += 2;

        if self.shrink == 128 {
            assert_eq!(self.flags, 0);
            if let Some(x) = self.more_flags.pop() {
                self.shrink = 64;
                self.flags = x as u128;
            }
        }
    }

    pub fn put_lambda(&mut self) {
        self.extend();
        self.flags |= 1;
    }

    pub fn put_app(&mut self) {
        self.extend();
        self.extend();
        self.flags |= 14;
    }

    pub fn pop_flag(&mut self) -> u8 {
        let result = (self.flags & 3) as u8;
        self.shrink();
        result
    }

    pub const LAMBDA_FLAG: u8 = 1;
    pub const APP_FLAG_MIDDLE: u8 = 2;
    pub const APP_FLAG_END: u8 = 3;
}

#[cfg(test)]
mod tests {
    use super::Flags;

    #[test]
    fn initial() {
        let mut initial = Flags::make();
        assert!(!initial.is_empty());
        let x = initial.pop_flag();
        assert!(x == Flags::APP_FLAG_MIDDLE);
        assert!(initial.is_empty());
    }

    fn pretty(f: &Flags) -> String {
        let r = f.more_flags.iter().map(|x| hex::encode(x.to_be_bytes()));
        let s = r.fold(String::new(), |mut a, s| {
            a.extend(s.chars());
            a
        });

        format!(
            "{}, shrink={}, {}",
            hex::encode(f.flags.to_be_bytes()),
            f.shrink,
            s
        )
    }

    #[test]
    fn extend_eq_shrink() {
        let mut flags = Flags::make();
        for _i in 1..65 {
            println!("{:?}", pretty(&flags));
            flags.put_lambda();
        }
        for i in 1..65 {
            println!("{:?}", pretty(&flags));
            assert_eq!(
                flags.pop_flag(),
                Flags::LAMBDA_FLAG,
                "test popping after {}th time",
                i
            );
        }
        assert_eq!(flags.pop_flag(), Flags::APP_FLAG_MIDDLE);
    }
}
