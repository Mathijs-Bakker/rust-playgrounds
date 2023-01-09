// A tait decouples function definitions so it can be used on multiple structs.
// this is how Rust achieves polymorphism
pub trait SomeTrait {
    fn is_valid(&self) -> bool;
}

pub struct RandomStruct {
    pub call_count: u64,
    pub some_bool: bool,
    pub some_int: u32,
}
impl SomeTrait for RandomStruct {
    fn is_valid(&self) -> bool {
        self.some_bool
    }
}

impl RandomStruct {
    pub fn new(param_a: bool) -> Self {
        Self {
            call_count: 0,
            some_bool: !param_a,
            some_int: 8,
        }
    }

    pub fn is_bigger(&mut self, compare_to: u32) -> bool {
        self.call_count += 1;
        self.some_int > compare_to
    }
}
