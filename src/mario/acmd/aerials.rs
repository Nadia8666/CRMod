#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*
};


unsafe extern "C" fn game_specialhi(fighter: &mut L2CAgentBase) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    macros::SA_SET(fighter, *SITUATION_KIND_AIR);

    macros::ATTACK(
        fighter,
        0, // id
        0, // part
        Hash40::new("hip"), // bone
        58.0, // dmg
        90, // ang
        200, // kbg
        0, // fkb
        200, // bkb
        15.0, // size
        0.0, // hitboxstart X
        6.0, // hitboxstart Y
        0.0, // hitboxstart Z
        Some(0.0), // hitboxend X
        Some(10.0), // hitboxend Y
        Some(0.0),  // hitboxend Z
        1.0, // hitlag
        1.0, // sdi
        *ATTACK_SETOFF_KIND_OFF, // ignore
        *ATTACK_LR_CHECK_POS,  // ignore
        false,  // knockback affected by weight
        3, // extra dmg to shield 2.1 = all
        0.0, // trip chance
        0, // rehit
        false, // reflectable
        false, // absorbable
        false, // flinchless
        false, // disable hitlag
        true, // direct hitbox (hitbox from the fighter, false if its from a weapon or smth)
        *COLLISION_SITUATION_MASK_GA, // ground or air only, G = GROUND | A = AIR | GA = BOTH
        *COLLISION_CATEGORY_MASK_ALL, // ignore
        *COLLISION_PART_MASK_ALL, // ignore
        false,  // friendly fire
        Hash40::new("collision_attr_normal"), // effect https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv collision_attr
        *ATTACK_SOUND_LEVEL_L, // sfx vol S/M/L
        *COLLISION_SOUND_ATTR_KICK, // sfx type ignore most time
        *ATTACK_REGION_THROW // hitbox type ignore
    );


    // Send player high up over time
    for _ in 0..10 {
        if macros::is_excute(fighter) {
            //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(fighter.lua_state_agent, 10.0);
    }

    // Transition
    wait(fighter.lua_state_agent, 10.0);

    // Set baack to fall state
    if macros::is_excute(fighter) {
        // Give back jumps
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);

        // Do jump effects
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING);
       
        // Whatever the fuck this is
        //0xe1ba0(false, true);
        
        // Sound
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_mario_special_h03"));
            macros::PLAY_SE(fighter, Hash40::new("se_mario_jump01"));
        }
    }
}


unsafe extern "C" fn game_specialhi_blank(_agent: &mut L2CAgentBase) {}

pub fn install(agent: &mut smashline::Agent) {
    //agent.game_acmd("game_attackairf", mario_attackairf);
    //agent.effect_acmd("effect_attackairf", mario_attackairf_eff);

    // DOES NOT WORK agent.game_acmd("game_attackspecialhi", game_specialhi);
    agent.game_acmd("game_specialhi", game_specialhi);
    agent.game_acmd("game_specialairhi", game_specialhi);

    agent.game_acmd("effect_specialairhi", game_specialhi_blank);
    agent.game_acmd("effect_specialhi", game_specialhi_blank);

    agent.game_acmd("sound_specialairhi", game_specialhi_blank);
    agent.game_acmd("sound_specialhi", game_specialhi_blank);

    agent.game_acmd("expression_specialairhi", game_specialhi_blank);
    agent.game_acmd("expression_specialhi", game_specialhi_blank);
}