#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
mod aerials;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
}