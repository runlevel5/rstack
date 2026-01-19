use libc::{c_char, c_int, c_void, size_t, ucontext_t};

use crate::*;

// UNW_TDEP_CURSOR_LEN is 280 in libunwind-ppc64.h
pub const UNW_TDEP_CURSOR_LEN: c_int = 280;

pub type unw_word_t = u64;
pub type unw_sword_t = i64;

// General purpose registers R0-R31
pub const UNW_PPC64_R0: c_int = 0;
pub const UNW_PPC64_R1: c_int = 1; // Stack pointer
pub const UNW_PPC64_R2: c_int = 2;
pub const UNW_PPC64_R3: c_int = 3;
pub const UNW_PPC64_R4: c_int = 4;
pub const UNW_PPC64_R5: c_int = 5;
pub const UNW_PPC64_R6: c_int = 6;
pub const UNW_PPC64_R7: c_int = 7;
pub const UNW_PPC64_R8: c_int = 8;
pub const UNW_PPC64_R9: c_int = 9;
pub const UNW_PPC64_R10: c_int = 10;
pub const UNW_PPC64_R11: c_int = 11; // Static chain
pub const UNW_PPC64_R12: c_int = 12;
pub const UNW_PPC64_R13: c_int = 13;
pub const UNW_PPC64_R14: c_int = 14;
pub const UNW_PPC64_R15: c_int = 15;
pub const UNW_PPC64_R16: c_int = 16;
pub const UNW_PPC64_R17: c_int = 17;
pub const UNW_PPC64_R18: c_int = 18;
pub const UNW_PPC64_R19: c_int = 19;
pub const UNW_PPC64_R20: c_int = 20;
pub const UNW_PPC64_R21: c_int = 21;
pub const UNW_PPC64_R22: c_int = 22;
pub const UNW_PPC64_R23: c_int = 23;
pub const UNW_PPC64_R24: c_int = 24;
pub const UNW_PPC64_R25: c_int = 25;
pub const UNW_PPC64_R26: c_int = 26;
pub const UNW_PPC64_R27: c_int = 27;
pub const UNW_PPC64_R28: c_int = 28;
pub const UNW_PPC64_R29: c_int = 29;
pub const UNW_PPC64_R30: c_int = 30;
pub const UNW_PPC64_R31: c_int = 31; // Hard frame pointer

// Floating point registers F0-F31
pub const UNW_PPC64_F0: c_int = 32;
pub const UNW_PPC64_F1: c_int = 33;
pub const UNW_PPC64_F2: c_int = 34;
pub const UNW_PPC64_F3: c_int = 35;
pub const UNW_PPC64_F4: c_int = 36;
pub const UNW_PPC64_F5: c_int = 37;
pub const UNW_PPC64_F6: c_int = 38;
pub const UNW_PPC64_F7: c_int = 39;
pub const UNW_PPC64_F8: c_int = 40;
pub const UNW_PPC64_F9: c_int = 41;
pub const UNW_PPC64_F10: c_int = 42;
pub const UNW_PPC64_F11: c_int = 43;
pub const UNW_PPC64_F12: c_int = 44;
pub const UNW_PPC64_F13: c_int = 45;
pub const UNW_PPC64_F14: c_int = 46;
pub const UNW_PPC64_F15: c_int = 47;
pub const UNW_PPC64_F16: c_int = 48;
pub const UNW_PPC64_F17: c_int = 49;
pub const UNW_PPC64_F18: c_int = 50;
pub const UNW_PPC64_F19: c_int = 51;
pub const UNW_PPC64_F20: c_int = 52;
pub const UNW_PPC64_F21: c_int = 53;
pub const UNW_PPC64_F22: c_int = 54;
pub const UNW_PPC64_F23: c_int = 55;
pub const UNW_PPC64_F24: c_int = 56;
pub const UNW_PPC64_F25: c_int = 57;
pub const UNW_PPC64_F26: c_int = 58;
pub const UNW_PPC64_F27: c_int = 59;
pub const UNW_PPC64_F28: c_int = 60;
pub const UNW_PPC64_F29: c_int = 61;
pub const UNW_PPC64_F30: c_int = 62;
pub const UNW_PPC64_F31: c_int = 63;

// Special registers
pub const UNW_PPC64_LR: c_int = 65; // Link register
pub const UNW_PPC64_CTR: c_int = 66; // Count register
pub const UNW_PPC64_ARG_POINTER: c_int = 67;

// Condition registers CR0-CR7
pub const UNW_PPC64_CR0: c_int = 68;
pub const UNW_PPC64_CR1: c_int = 69;
pub const UNW_PPC64_CR2: c_int = 70;
pub const UNW_PPC64_CR3: c_int = 71;
pub const UNW_PPC64_CR4: c_int = 72;
pub const UNW_PPC64_CR5: c_int = 73;
pub const UNW_PPC64_CR6: c_int = 74;
pub const UNW_PPC64_CR7: c_int = 75;

pub const UNW_PPC64_XER: c_int = 76;

// Vector registers V0-V31 (AltiVec)
pub const UNW_PPC64_V0: c_int = 77;
pub const UNW_PPC64_V1: c_int = 78;
pub const UNW_PPC64_V2: c_int = 79;
pub const UNW_PPC64_V3: c_int = 80;
pub const UNW_PPC64_V4: c_int = 81;
pub const UNW_PPC64_V5: c_int = 82;
pub const UNW_PPC64_V6: c_int = 83;
pub const UNW_PPC64_V7: c_int = 84;
pub const UNW_PPC64_V8: c_int = 85;
pub const UNW_PPC64_V9: c_int = 86;
pub const UNW_PPC64_V10: c_int = 87;
pub const UNW_PPC64_V11: c_int = 88;
pub const UNW_PPC64_V12: c_int = 89;
pub const UNW_PPC64_V13: c_int = 90;
pub const UNW_PPC64_V14: c_int = 91;
pub const UNW_PPC64_V15: c_int = 92;
pub const UNW_PPC64_V16: c_int = 93;
pub const UNW_PPC64_V17: c_int = 94;
pub const UNW_PPC64_V18: c_int = 95;
pub const UNW_PPC64_V19: c_int = 96;
pub const UNW_PPC64_V20: c_int = 97;
pub const UNW_PPC64_V21: c_int = 98;
pub const UNW_PPC64_V22: c_int = 99;
pub const UNW_PPC64_V23: c_int = 100;
pub const UNW_PPC64_V24: c_int = 101;
pub const UNW_PPC64_V25: c_int = 102;
pub const UNW_PPC64_V26: c_int = 103;
pub const UNW_PPC64_V27: c_int = 104;
pub const UNW_PPC64_V28: c_int = 105;
pub const UNW_PPC64_V29: c_int = 106;
pub const UNW_PPC64_V30: c_int = 107;
pub const UNW_PPC64_V31: c_int = 108;

pub const UNW_PPC64_VRSAVE: c_int = 109;
pub const UNW_PPC64_VSCR: c_int = 110;
pub const UNW_PPC64_SPE_ACC: c_int = 111;
pub const UNW_PPC64_SPEFSCR: c_int = 112;

// Frame info (read-only)
pub const UNW_PPC64_FRAME_POINTER: c_int = 113;
pub const UNW_PPC64_NIP: c_int = 114; // Next instruction pointer

// Target-dependent definitions
pub const UNW_TDEP_LAST_REG: c_int = UNW_PPC64_NIP;
pub const UNW_TDEP_IP: c_int = UNW_PPC64_NIP;
pub const UNW_TDEP_SP: c_int = UNW_PPC64_R1;
pub const UNW_TDEP_EH: c_int = UNW_PPC64_R12;

pub const UNW_TDEP_NUM_EH_REGS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_tdep_save_loc_t {
    pub unused: c_char,
}

// On ppc64, we can directly use ucontext_t as the unwind context
pub type unw_tdep_context_t = ucontext_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_tdep_proc_info_t {
    pub unused: c_char,
}

// On ppc64, libunwind uses getcontext() from libc
#[macro_export]
macro_rules! unw_tdep_getcontext {
    ($uc:expr) => {{
        extern "C" {
            fn getcontext(ucp: *mut libc::ucontext_t) -> libc::c_int;
        }
        getcontext($uc as *mut libc::ucontext_t);
        0
    }};
}

extern "C" {
    #[link_name = "_Uppc64_init_local"]
    pub fn unw_init_local(cur: *mut unw_cursor_t, ctx: *mut unw_context_t) -> c_int;

    #[link_name = "_Uppc64_init_remote"]
    pub fn unw_init_remote(cur: *mut unw_cursor_t, spc: unw_addr_space_t, p: *mut c_void) -> c_int;

    #[link_name = "_Uppc64_step"]
    pub fn unw_step(cur: *mut unw_cursor_t) -> c_int;

    #[link_name = "_Uppc64_get_reg"]
    pub fn unw_get_reg(cur: *mut unw_cursor_t, reg: unw_regnum_t, valp: *mut unw_word_t) -> c_int;

    #[link_name = "_Uppc64_set_reg"]
    pub fn unw_set_reg(cur: *mut unw_cursor_t, reg: unw_regnum_t, val: unw_word_t) -> c_int;

    #[link_name = "_Uppc64_resume"]
    pub fn unw_resume(cur: *mut unw_cursor_t) -> c_int;

    #[link_name = "_Uppc64_create_addr_space"]
    pub fn unw_create_addr_space(
        accessors: *mut unw_accessors_t,
        byteorder: c_int,
    ) -> unw_addr_space_t;

    #[link_name = "_Uppc64_destroy_addr_space"]
    pub fn unw_destroy_addr_space(spc: unw_addr_space_t);

    #[link_name = "_Uppc64_get_accessors"]
    pub fn unw_get_accessors(spc: unw_addr_space_t) -> *mut unw_accessors_t;

    #[link_name = "_Uppc64_flush_cache"]
    pub fn unw_flush_cache(spc: unw_addr_space_t, lo: unw_word_t, hi: unw_word_t);

    #[link_name = "_Uppc64_set_caching_policy"]
    pub fn unw_set_caching_policy(spc: unw_addr_space_t, policy: unw_caching_policy_t) -> c_int;

    #[link_name = "_Uppc64_regname"]
    pub fn unw_regname(reg: unw_regnum_t) -> *const c_char;

    #[link_name = "_Uppc64_get_proc_info"]
    pub fn unw_get_proc_info(cur: *mut unw_cursor_t, info: *mut unw_proc_info_t) -> c_int;

    #[link_name = "_Uppc64_get_save_loc"]
    pub fn unw_get_save_loc(cur: *mut unw_cursor_t, a: c_int, p: *mut unw_save_loc_t) -> c_int;

    #[link_name = "_Uppc64_is_fpreg"]
    pub fn unw_tdep_is_fpreg(reg: unw_regnum_t) -> c_int;

    #[link_name = "_Uppc64_is_signal_frame"]
    pub fn unw_is_signal_frame(cur: *mut unw_cursor_t) -> c_int;

    #[link_name = "_Uppc64_get_proc_name"]
    pub fn unw_get_proc_name(
        cur: *mut unw_cursor_t,
        buf: *mut c_char,
        len: size_t,
        offp: *mut unw_word_t,
    ) -> c_int;

    #[link_name = "_Uppc64_strerror"]
    pub fn unw_strerror(err_code: c_int) -> *const c_char;

    #[link_name = "_Uppc64_local_addr_space"]
    pub static unw_local_addr_space: unw_addr_space_t;
}
