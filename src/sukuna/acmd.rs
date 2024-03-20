#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
mod aerials;
mod grabs;
mod specials;
mod lights;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    grabs::install(agent);
    lights::install(agent);
    specials::install(agent);
}