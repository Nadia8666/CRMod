#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use smash::lib::LuaConst;
use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    crate::custom::global_fighter_frame,
    std::collections::HashMap
};

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.

static mut USED_SPECIAL_FALL: [bool; 8] = [false; 8];

unsafe extern "C" fn mario_on_main(fighter: &mut L2CFighterCommon) {
    // i hate hashmaps.
    let mut reset_hash:HashMap<i32, bool> = HashMap::new();

    reset_hash.insert(*FIGHTER_STATUS_KIND_LANDING, true);
    reset_hash.insert(*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, true);
    reset_hash.insert(*FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, true);
    reset_hash.insert(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
    reset_hash.insert(*FIGHTER_STATUS_KIND_LANDING_LIGHT, true);
    reset_hash.insert(*FIGHTER_STATUS_KIND_WAIT, true);
    reset_hash.insert(*FIGHTER_STATUS_KIND_WALK, true);

    unsafe {
        // Remove special fall
        let boma = fighter.module_accessor;
        let chr_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let reset_fall_state = reset_hash.get(&StatusModule::status_kind(boma));
        
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            // Change status to regular fall, and set jumps to 0
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            
            // Remove jumping, disables up b
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) != 1 {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }

            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);

            // Limit up b usage
            USED_SPECIAL_FALL[chr_id] = true
        } else if reset_fall_state.is_some() && USED_SPECIAL_FALL[chr_id] {
            // Restore jumps, reenabling up b
            //WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);

            // If landed remove previous special fall
            USED_SPECIAL_FALL[chr_id] = false
        }
        
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            // Restore jumps, reenabling up b
            let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor);
            if stick_x != 0.0 {
                if stick_x < 0.0 {
                    stick_x = -1.0
                } else {
                    stick_x = 1.0
                }

                PostureModule::set_lr(fighter.module_accessor, stick_x);
            }    
            
            // Make animation virtually instant and stop any sounds it plays
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) == 1 {
                macros::FT_MOTION_RATE(fighter, 0.0002);
                macros::STOP_SE(fighter, smashline::Hash40::new("se_mario_special_h03"));
            }

            // Force fall animation if used multiple times in one fall
            /*  crashes?
            let valid_anim = MotionModule::motion_kind(boma) == smashline::Hash40::new("special_hi").hash || MotionModule::motion_kind(boma) == smashline::Hash40::new("special_air_hi").hash;
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) == 1 && valid_anim {
                MotionModule::change_motion(fighter.module_accessor,
                    smashline::Hash40::new("fall"), // animation
                    0.0, // start frame
                    1.0, // speed
                    false, // looping
                    0.0, // amcd/hitbox code frame start
                    false, // false
                    false // false
                );

                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            */
        }

        // Calls the global fighter frame
        global_fighter_frame(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, mario_on_main);
}