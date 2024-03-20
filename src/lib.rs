#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]

mod mario;
mod custom;
mod sukuna;

#[skyline::main(name = "crmod")]
pub fn main() {
    sukuna::install();
    mario::install();
}