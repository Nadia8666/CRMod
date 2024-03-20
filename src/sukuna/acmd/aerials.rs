#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
use {
    skyline::{hooks::{getRegionAddress, Region}, libc::*, nn::ro::LookupSymbol}, smash::{
        app::{lua_bind::*, sv_animcmd::*, sv_battle_object, utility}, lib::lua_const::*, lua2cpp::*, phx::*
    }, smash_script::*
};

unsafe extern "C" fn forward_air(fighter: &mut L2CAgentBase) {
    let dir = PostureModule::lr(fighter.module_accessor);
    
    for _ in 0..5 {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: 0.65, y: 0.0, z: 0.0});
        wait(fighter.lua_state_agent, 2.0);
    }

    shield!(
        fighter,
        *MA_MSC_CMD_REFLECTOR,
        *COLLISION_KIND_REFLECTOR,
        0,
        Hash40::new("hip"),
        8.5,
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        2.0,
        2.0,
        998,
        false,
        2.0,
        *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
    );

    macros::ATTACK(
        fighter,
        0, // id
        0, // part
        Hash40::new("hip"), // bone
        10.0, // dmg
        25, // ang
        30, // kbg
        0, // fkb
        75, // bkb
        8.5, // size
        0.0, // hitboxstart X
        0.0, // hitboxstart Y
        5.0, // hitboxstart Z
        Some(0.0), // hitboxend X
        Some(0.0), // hitboxend Y
        Some(0.0),  // hitboxend Z
        2.0, // hitlag
        0.0, // sdi
        *ATTACK_SETOFF_KIND_OFF, // ignore
        *ATTACK_LR_CHECK_POS,  // ignore
        false,  // knockback affected by weight
        1.65, // extra dmg to shield 2.1 = all
        0.0, // trip chance
        0, // rehit
        true, // reflectable
        false, // absorbable
        false, // flinchless
        false, // disable hitlag
        true, // direct hitbox (hitbox from the fighter, false if its from a weapon or smth)
        *COLLISION_SITUATION_MASK_GA, // ground or air only, G = GROUND | A = AIR | GA = BOTH
        *COLLISION_CATEGORY_MASK_ALL, // ignore
        *COLLISION_PART_MASK_ALL, // ignore
        true,  // friendly fire
        Hash40::new("collision_attr_normal"), // effect https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv collision_attr
        *ATTACK_SOUND_LEVEL_L, // sfx vol S/M/L
        *COLLISION_SOUND_ATTR_KICK, // sfx type ignore most time
        *ATTACK_REGION_PUNCH // hitbox type ignore
    );

    wait(fighter.lua_state_agent, 20.0);
    AttackModule::clear_all(fighter.module_accessor);
    shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_attackairf", forward_air);
}