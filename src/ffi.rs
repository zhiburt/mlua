#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ptr;
use std::os::raw::*;

pub type lua_Integer = c_longlong;
pub type lua_Number = c_double;

pub enum lua_State {}
pub type lua_Alloc = extern "C" fn(ud: *mut c_void,
                                   ptr: *mut c_void,
                                   osize: usize,
                                   nsize: usize)
                                   -> *mut c_void;
pub type lua_KContext = *mut c_void;
pub type lua_KFunction = unsafe extern "C" fn(state: *mut lua_State,
                                              status: c_int,
                                              ctx: lua_KContext)
                                              -> c_int;
pub type lua_CFunction = unsafe extern "C" fn(state: *mut lua_State) -> c_int;

pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRGCMM: c_int = 5;
pub const LUA_ERRERR: c_int = 6;

pub const LUA_NOREF: c_int = -2;
pub const LUA_REFNIL: c_int = -1;

pub const LUA_MULTRET: c_int = -1;
pub const LUAI_MAXSTACK: c_int = 1000000;
pub const LUA_REGISTRYINDEX: c_int = -LUAI_MAXSTACK - 1000;
pub const LUA_RIDX_MAINTHREAD: lua_Integer = 1;
pub const LUA_RIDX_GLOBALS: lua_Integer = 2;

pub const LUA_TNONE: c_int = -1;
pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

extern "C" {
    pub fn lua_close(state: *mut lua_State);
    pub fn lua_callk(
        state: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        ctx: lua_KContext,
        k: Option<lua_KFunction>,
    );
    pub fn lua_pcallk(
        state: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        msgh: c_int,
        ctx: lua_KContext,
        k: Option<lua_KFunction>,
    ) -> c_int;
    pub fn lua_resume(state: *mut lua_State, from: *mut lua_State, nargs: c_int) -> c_int;
    pub fn lua_status(state: *mut lua_State) -> c_int;

    pub fn lua_pushnil(state: *mut lua_State);
    pub fn lua_pushvalue(state: *mut lua_State, index: c_int);
    pub fn lua_pushboolean(state: *mut lua_State, b: c_int);
    pub fn lua_pushinteger(state: *mut lua_State, n: lua_Integer);
    pub fn lua_pushnumber(state: *mut lua_State, n: lua_Number);
    pub fn lua_pushlstring(state: *mut lua_State, s: *const c_char, len: usize) -> *const c_char;
    pub fn lua_pushlightuserdata(state: *mut lua_State, data: *mut c_void);
    pub fn lua_pushcclosure(state: *mut lua_State, function: lua_CFunction, n: c_int);

    pub fn lua_tointegerx(state: *mut lua_State, index: c_int, isnum: *mut c_int) -> lua_Integer;
    pub fn lua_tolstring(state: *mut lua_State, index: c_int, len: *mut usize) -> *const c_char;
    pub fn lua_toboolean(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_tonumberx(state: *mut lua_State, index: c_int, isnum: *mut c_int) -> lua_Number;
    pub fn lua_touserdata(state: *mut lua_State, index: c_int) -> *mut c_void;
    pub fn lua_tothread(state: *mut lua_State, index: c_int) -> *mut lua_State;

    pub fn lua_gettop(state: *const lua_State) -> c_int;
    pub fn lua_settop(state: *mut lua_State, n: c_int);
    pub fn lua_checkstack(state: *mut lua_State, n: c_int) -> c_int;
    pub fn lua_rotate(state: *mut lua_State, index: c_int, n: c_int);
    pub fn lua_copy(state: *mut lua_State, from: c_int, to: c_int);
    pub fn lua_absindex(state: *mut lua_State, index: c_int) -> c_int;

    pub fn lua_isinteger(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isnumber(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isstring(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_iscfunction(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_isuserdata(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_type(state: *mut lua_State, index: c_int) -> c_int;

    pub fn lua_gettable(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_geti(state: *mut lua_State, index: c_int, i: lua_Integer) -> c_int;
    pub fn lua_rawget(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_rawgeti(state: *mut lua_State, index: c_int, n: lua_Integer) -> c_int;
    pub fn lua_getmetatable(state: *mut lua_State, index: c_int) -> c_int;

    pub fn lua_createtable(state: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn lua_newuserdata(state: *mut lua_State, size: usize) -> *mut c_void;
    pub fn lua_newthread(state: *mut lua_State) -> *mut lua_State;

    pub fn lua_settable(state: *mut lua_State, index: c_int);
    pub fn lua_rawset(state: *mut lua_State, index: c_int);
    pub fn lua_setmetatable(state: *mut lua_State, index: c_int);

    pub fn lua_len(state: *mut lua_State, index: c_int);
    pub fn lua_rawlen(state: *mut lua_State, index: c_int) -> usize;
    pub fn lua_next(state: *mut lua_State, index: c_int) -> c_int;
    pub fn lua_rawequal(state: *mut lua_State, index1: c_int, index2: c_int) -> c_int;

    pub fn lua_error(state: *mut lua_State) -> !;
    pub fn lua_atpanic(state: *mut lua_State, panic: lua_CFunction) -> lua_CFunction;

    pub fn luaL_newstate() -> *mut lua_State;
    pub fn luaL_openlibs(state: *mut lua_State);
    pub fn luaL_loadbufferx(
        state: *mut lua_State,
        buf: *const c_char,
        size: usize,
        name: *const c_char,
        mode: *const c_char,
    ) -> c_int;
    pub fn luaL_ref(state: *mut lua_State, table: c_int) -> c_int;
    pub fn luaL_unref(state: *mut lua_State, table: c_int, lref: c_int);
    pub fn luaL_checkstack(state: *mut lua_State, size: c_int, msg: *const c_char);
    pub fn luaL_traceback(
        push_state: *mut lua_State,
        state: *mut lua_State,
        msg: *const c_char,
        level: c_int,
    );
    pub fn luaL_len(push_state: *mut lua_State, index: c_int) -> lua_Integer;
}

pub unsafe fn lua_pop(state: *mut lua_State, n: c_int) {
    lua_settop(state, -n - 1);
}

pub unsafe fn lua_newtable(state: *mut lua_State) {
    lua_createtable(state, 0, 0);
}

pub fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_REGISTRYINDEX - i
}

pub unsafe fn lua_pushcfunction(state: *mut lua_State, function: lua_CFunction) {
    lua_pushcclosure(state, function, 0);
}

pub unsafe fn lua_tonumber(state: *mut lua_State, index: c_int) -> lua_Number {
    lua_tonumberx(state, index, ptr::null_mut())
}

pub unsafe fn lua_tointeger(state: *mut lua_State, index: c_int) -> lua_Integer {
    lua_tointegerx(state, index, ptr::null_mut())
}

pub unsafe fn lua_tostring(state: *mut lua_State, index: c_int) -> *const c_char {
    lua_tolstring(state, index, ptr::null_mut())
}

pub unsafe fn lua_isfunction(state: *mut lua_State, index: c_int) -> c_int {
    if lua_type(state, index) == LUA_TFUNCTION {
        1
    } else {
        0
    }
}

pub unsafe fn lua_istable(state: *mut lua_State, index: c_int) -> c_int {
    if lua_type(state, index) == LUA_TTABLE {
        1
    } else {
        0
    }
}

pub unsafe fn lua_islightuserdata(state: *mut lua_State, index: c_int) -> c_int {
    if lua_type(state, index) == LUA_TLIGHTUSERDATA {
        1
    } else {
        0
    }
}

pub unsafe fn lua_isnil(state: *mut lua_State, index: c_int) -> c_int {
    if lua_type(state, index) == LUA_TNIL {
        1
    } else {
        0
    }
}

pub unsafe fn lua_isboolean(state: *mut lua_State, index: c_int) -> c_int {
    if lua_type(state, index) == LUA_TBOOLEAN {
        1
    } else {
        0
    }
}

pub unsafe fn lua_isthread(state: *mut lua_State, index: c_int) -> c_int {
    if lua_type(state, index) == LUA_TTHREAD {
        1
    } else {
        0
    }
}

pub unsafe fn lua_isnone(state: *mut lua_State, index: c_int) -> c_int {
    if lua_type(state, index) == LUA_TNONE {
        1
    } else {
        0
    }
}

pub unsafe fn lua_insert(state: *mut lua_State, index: c_int) {
    lua_rotate(state, index, 1);
}

pub unsafe fn lua_remove(state: *mut lua_State, index: c_int) {
    lua_rotate(state, index, -1);
    lua_pop(state, 1);
}

pub unsafe fn lua_call(state: *mut lua_State, nargs: c_int, nresults: c_int) {
    lua_callk(state, nargs, nresults, ptr::null_mut(), None)
}

pub unsafe fn lua_pcall(
    state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    msgh: c_int,
) -> c_int {
    lua_pcallk(state, nargs, nresults, msgh, ptr::null_mut(), None)
}

pub unsafe fn lua_replace(state: *mut lua_State, index: c_int) {
    lua_copy(state, -1, index);
    lua_pop(state, 1);
}

pub unsafe fn luaL_loadbuffer(
    state: *mut lua_State,
    buf: *const c_char,
    size: usize,
    name: *const c_char,
) -> c_int {
    luaL_loadbufferx(state, buf, size, name, ptr::null())
}
