#![deny(clippy::all)]
#[cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::needless_pass_by_value)
)]
extern crate sharefem;

fn main() {
    sharefem::init_rocket().launch();
}