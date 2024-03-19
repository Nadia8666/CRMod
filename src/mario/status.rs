#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
mod special_n;

pub fn install(agent: &mut smashline::Agent) {
    special_n::install(agent);
}