#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
use smash::{
    lua2cpp::*,
    lib::{lua_const::*, L2CValue}
};

/*

unsafe extern "C" fn mario_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // calls the original status
    //let original = smashline::original_status(smashline::Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N);
    //original(fighter)
}
*/

pub fn install(_agent: &mut smashline::Agent) {
    //agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_N, mario_special_n_main);
}