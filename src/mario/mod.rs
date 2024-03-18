// mario

// mario_fireball
mod mariomovesetedit;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    agent.install();

    mariomovesetedit::install();
}
