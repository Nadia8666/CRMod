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

#[skyline::main(name = "crmod")]
pub fn main() {
    mario::install();
}