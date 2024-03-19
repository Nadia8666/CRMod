#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.

static mut USED_SPECIAL_FALL: [bool; 8] = [false; 8];

unsafe extern "C" fn mario_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        // Remove special fall
        let boma = fighter.module_accessor;
        let chr_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let mut should_call_frame:bool = true;

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            // Change status to regular fall, and set jumps to 0
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);

            // Limit up b usage
            USED_SPECIAL_FALL[chr_id] = true
        } else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI && USED_SPECIAL_FALL[chr_id] {
            // If previously special falled cancel current up b
            if should_call_frame {
                should_call_frame = false;
            }
            
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        } else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_LANDING && USED_SPECIAL_FALL[chr_id] {
            // If landed remove previous special fall
            USED_SPECIAL_FALL[chr_id] = false
        }

        // Calls the global fighter frame
        if should_call_frame {
            global_fighter_frame(fighter);
        }

        // Reset SCF
        should_call_frame = true;
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, mario_on_main);
}