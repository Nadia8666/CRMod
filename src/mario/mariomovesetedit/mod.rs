mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario_mariomovesetedit");
    acmd::install(agent);
    agent.install();
}