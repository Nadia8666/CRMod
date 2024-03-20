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

unsafe extern "C" fn side_special(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    // 17 Frame startup
    wait(fighter.lua_state_agent, 17.0);
    
    println!("attacking");
    
    macros::ATTACK(
        fighter,
        0, // id
        0, // part
        Hash40::new("hip"), // bone
    10.0, // dmg
        50, // ang
        45, // kbg
        0, // fkb
        60, // bkb
        10.0, // size
        0.0, // hitboxstart X
        2.0, // hitboxstart Y
        18.0, // hitboxstart Z
        Some(0.0), // hitboxend X
        Some(2.0), // hitboxend Y
        Some(0.0),  // hitboxend Z
        0.5, // hitlag
        0.0, // sdi
        *ATTACK_SETOFF_KIND_OFF, // ignore
        *ATTACK_LR_CHECK_POS,  // ignore
        false,  // knockback affected by weight
        1.1, // extra dmg to shield 2.1 = all
        0.2, // trip chance
        0, // rehit
        false, // reflectable
        false, // absorbable
        false, // flinchless
        false, // disable hitlag
        true, // direct hitbox (hitbox from the fighter, false if its from a weapon or smth)
        *COLLISION_SITUATION_MASK_GA, // ground or air only, G = GROUND | A = AIR | GA = BOTH
        *COLLISION_CATEGORY_MASK_ALL, // ignore
        *COLLISION_PART_MASK_ALL, // ignore
        true,  // friendly fire
        Hash40::new("collision_attr_normal"), // effect https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv collision_attr
        *ATTACK_SOUND_LEVEL_M, // sfx vol S/M/L
        *COLLISION_SOUND_ATTR_KICK, // sfx type ignore most time
        *ATTACK_REGION_PUNCH // hitbox type ignore
    );

    // Hitbox lasts 10 frames
    wait(fighter.lua_state_agent, 10.0);
    AttackModule::clear_all(fighter.module_accessor);
}


unsafe extern "C" fn side_special_blank(_fighter: &mut L2CAgentBase) {
    
}


pub fn install(agent: &mut smashline::Agent) {
    // Side B
    agent.game_acmd("game_specials1", side_special);
    agent.game_acmd("game_specials2", side_special);
    agent.game_acmd("game_specialairs1", side_special);
    agent.game_acmd("game_specialairs2", side_special);

    agent.game_acmd("sound_specials1", side_special_blank);
    agent.game_acmd("sound_specials2", side_special_blank);
    agent.game_acmd("sound_specialairs1", side_special_blank);
    agent.game_acmd("sound_specialairs2", side_special_blank);

    agent.game_acmd("effect_specials1", side_special_blank);
    agent.game_acmd("effect_specials2", side_special_blank);
    agent.game_acmd("effect_specialairs1", side_special_blank);
    agent.game_acmd("effect_specialairs2", side_special_blank);


    // Up B
}