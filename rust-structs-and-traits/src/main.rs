mod i_am_not_main;
use crate::i_am_not_main::*;

#[allow(dead_code)]
struct MyStruct {
    some_bool: bool,
    some_float: f32,
    some_int: u32,
    random_data: RandomStruct,
}

impl RandomStruct {
    pub fn is_smaller(&self, compare_to: u32) -> bool {
        self.some_int < compare_to
    }
}

impl SomeTrait for MyStruct {
    fn is_valid(&self) -> bool {
        true
    }
}

fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Whoot! {:?}", check_me.is_valid());
    }
}

#[allow(unused_variables)]
fn main() {
    let mut random_struct: RandomStruct = RandomStruct {
        some_bool: false,
        some_int: 32,
        call_count: 0,
    };

    let mut my_var = MyStruct {
        some_bool: true,
        some_float: 2.0,
        some_int: 20,
        random_data: RandomStruct::new(true),
    };

    my_var.some_bool = false;

    let my_var_2 = MyStruct {
        some_float: 2.4,
        ..my_var
    };

    let is_bigger = random_struct.is_bigger(3);
    let is_smaller = random_struct.is_smaller(2);
    let is_valid = random_struct.is_valid();

    print_if_is_valid(&random_struct);
    print_if_is_valid(&my_var_2);
}
