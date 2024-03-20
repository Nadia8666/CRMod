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

pub fn install(agent: &mut smashline::Agent) {

}