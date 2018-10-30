#![deny(clippy::all)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
extern crate femapi;

fn main() {
    femapi::init_rocket().launch();
}
