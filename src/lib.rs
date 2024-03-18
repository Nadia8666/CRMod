#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod mario;
mod custom;

#[skyline::main(name = "crmod")]
pub fn main() {
    mario::install();
}