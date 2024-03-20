// sukuna, over joker
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
mod acmd;
mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("jack");
    acmd::install(agent);
    frame::install(agent);
    agent.install();
}
