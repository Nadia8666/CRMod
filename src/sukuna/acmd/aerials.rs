#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*,
};


unsafe extern "C" fn game_specialhi(fighter: &mut L2CAgentBase) {
    //AttackModule::clear_all(fighter.module_accessor);
    //macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
}

pub fn install(agent: &mut smashline::Agent) {
    //agent.game_acmd("game_attackairf", mario_attackairf);
    //agent.effect_acmd("effect_attackairf", mario_attackairf_eff);

    // DOES NOT WORK agent.game_acmd("game_attackspecialhi", game_specialhi);
    agent.game_acmd("game_specialhi", game_specialhi);
    agent.game_acmd("game_specialairhi", game_specialhi);
}