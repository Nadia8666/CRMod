#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use smash::lib::LuaConst;
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    crate::custom::global_fighter_frame,
    std::collections::HashMap
};

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn sukuna_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        // Get gauge
        let boma = fighter.module_accessor;
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

        if color == 3 {
            let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
            let rebel_gauge = WorkModule::get_float(fighter.module_accessor, 0x4D);

            // Passive heal
            if rebel_gauge > 10.0 {
                smash::app::FighterSpecializer_Jack::add_rebel_gauge(boma, smash::app::FighterEntryID(entry_id), -0.45);
                DamageModule::heal(boma, -0.25, 0);
            }

            println!("{}", WorkModule::get_float(boma, 0x4D));
        }

        // Calls the global fighter frame
        global_fighter_frame(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, sukuna_main);
}