extern crate safelog;
#[macro_use]
extern crate safelog_derive;

use safelog::Loggable;

#[derive(Loggable)]
struct TestLoggable {
    n_i32: i32,
    #[safelog(redact)]
    n_string: String,
}

fn main() {
    println!("Hello, world!");
}
