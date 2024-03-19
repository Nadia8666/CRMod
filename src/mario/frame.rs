#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.

static mut USED_SPECIAL_FALL: bool = false;

unsafe extern "C" fn mario_on_main(fighter: &mut L2CFighterCommon) {
   
    unsafe {
        // Remove special fall
        let boma = fighter.module_accessor;
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            // Change status to regular fall, and set jumps to 0
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);

            // Limit up b usage
            USED_SPECIAL_FALL = true
        } else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI && USED_SPECIAL_FALL {
            // If previously special falled cancel current up b
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        } else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_LANDING && USED_SPECIAL_FALL {
            // If landed remove previous special fall
            USED_SPECIAL_FALL = false
        }

        // Calls the global fighter frame
        global_fighter_frame(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, mario_on_main);
}