//! Low level bindings to Lua 5.4/5.3/5.2/5.1 (including LuaJIT) and Luau.

#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unsafe_op_in_unsafe_fn)]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::os::raw::c_int;

pub use luau::*;

pub const LUA_MAX_UPVALUES: c_int = 200;

// I believe `luaL_traceback` < 5.4 requires this much free stack to not error.
// 5.4 uses `luaL_Buffer`
#[doc(hidden)]
pub const LUA_TRACEBACK_STACK: c_int = 11;

// Copied from https://github.com/rust-lang/rust/blob/master/library/std/src/sys/pal/common/alloc.rs
// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values.
#[cfg(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "m68k",
    target_arch = "csky",
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc",
    target_arch = "wasm32",
    target_arch = "hexagon",
    all(target_arch = "riscv32", not(any(target_os = "espidf", target_os = "zkvm"))),
    all(target_arch = "xtensa", not(target_os = "espidf")),
))]
#[doc(hidden)]
pub const SYS_MIN_ALIGN: usize = 8;
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "loongarch64",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "s390x",
    target_arch = "sparc64",
    target_arch = "riscv64",
    target_arch = "wasm64",
))]
#[doc(hidden)]
pub const SYS_MIN_ALIGN: usize = 16;
// The allocator on the esp-idf and zkvm platforms guarantee 4 byte alignment.
#[cfg(any(
    all(target_arch = "riscv32", any(target_os = "espidf", target_os = "zkvm")),
    all(target_arch = "xtensa", target_os = "espidf"),
))]
#[doc(hidden)]
pub const SYS_MIN_ALIGN: usize = 4;

#[macro_use]
mod macros;

pub mod luau;
