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

    if color == 3 {  
        // wait(fighter.lua_state_agent, 10.0);
    
        println!("attacking");
    
        macros::ATTACK(
            fighter,
            0, // id
            0, // part
            Hash40::new("hip"), // bone
            8.0, // dmg
            50, // ang
            10, // kbg
            0, // fkb
            30, // bkb
            5.0, // size
            0.0, // hitboxstart X
            5.0, // hitboxstart Y
            13.0, // hitboxstart Z
            Some(0.0), // hitboxend X
            Some(5.0), // hitboxend Y
            Some(0.0),  // hitboxend Z
            2.0, // hitlag
            1.0, // sdi
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
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specials1", side_special);
    agent.game_acmd("game_specials2", side_special);
}