use std::{borrow::Cow, fmt::Display};

// Traits -> Defining Shared Behavior
// A trait defines functionality a particular type has and can share with other
// types. We can use traits to define shared behavior in an abstract way.
// We can use `trait bounds` to specify that a generic type can be any type
// that has certain behavior.

fn main() {
    let msg: &str = "Status: Operational";

    let msg_2: String = "Status: Error".to_string();
    let msg_3: Cow<str> = Cow::from(msg_2.clone());

    print_report_status(msg);
    print_report_status(msg_2);
    print_report_status(msg_3);
}

// Monomorphization
// monomorphization is a compile-time process where polymorphic functions are
// replaced by many monomorphic functions for each unique instantiation.
// This transformation is desirable, since then the output intermediate
// representation (IR) will have concrete types and can be optimized better.
// https://en.wikipedia.org/wiki/Monomorphization

fn print_report_status<T: AsRef<str>>(report: T) {
    let txt: String = report.as_ref().to_string();
    let text: &dyn Display = if txt.is_empty() {
        &"Invalid Input"
    } else {
        &txt
    };

    println!("{}", report.as_ref());
}
