# ulua
[![Build Status]][github-actions] [![Latest Version]][crates.io] [![API Documentation]][docs.rs] [![Coverage Status]][codecov.io] ![MSRV]

[Build Status]: https://github.com/rhodem-labs/ulua/workflows/CI/badge.svg
[github-actions]: https://github.com/rhodem-labs/ulua/actions
[Latest Version]: https://img.shields.io/crates/v/ulua.svg
[crates.io]: https://crates.io/crates/ulua
[API Documentation]: https://docs.rs/ulua/badge.svg
[docs.rs]: https://docs.rs/ulua
[Coverage Status]: https://codecov.io/gh/ulua/branch/main/graph/badge.svg?token=99339FS1CG
[codecov.io]: https://codecov.io/gh/rhodem-labs/ulua
[MSRV]: https://img.shields.io/badge/rust-1.79+-brightgreen.svg?&logo=rust

`ulua` is a set of bindings to the [Lua](https://www.lua.org) programming language for Rust with a goal to provide a
_safe_ (as much as possible), high level, easy to use, practical and flexible API.

`ulua` is tested on Windows/macOS/Linux including module mode in [GitHub Actions] on `x86_64` platforms and cross-compilation to `aarch64` (other targets are also supported).

WebAssembly (WASM) is supported through `wasm32-unknown-emscripten` target for all Lua/Luau versions excluding JIT.

[GitHub Actions]: https://github.com/rhodem-labs/ulua/actions
[Luau]: https://luau.org

## Usage

### Feature flags

`ulua` uses feature flags to reduce the amount of dependencies and compiled code, and allow to choose only required set of features.
Below is a list of the available feature flags. By default `ulua` does not enable any features.

* `vector4`: enable [Luau]'s 4-dimensional vector.
* `async`: enable async/await support (any executor can be used, eg. [tokio] or [async-std])
* `send`: make `ulua::Lua: Send + Sync` (adds [`Send`] requirement to `ulua::Function` and `ulua::UserData`)
* `error-send`: make `ulua:Error: Send + Sync`
* `serde`: add serialization and deserialization support to `ulua` types using [serde]
* `macros`: enable procedural macros (such as `chunk!`)
* `anyhow`: enable `anyhow::Error` conversion into Lua
* `userdata-wrappers`: opt into `impl UserData` for `Rc<T>`/`Arc<T>`/`Rc<RefCell<T>>`/`Arc<Mutex<T>>` where `T: UserData`

[Luau]: https://github.com/luau-lang/luau
[tokio]: https://github.com/tokio-rs/tokio
[async-std]: https://github.com/async-rs/async-std
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[serde]: https://github.com/serde-rs/serde

### Async/await support

`ulua` supports async/await using Lua [coroutines](https://www.lua.org/manual/5.3/manual.html#2.6) and requires running [Thread](https://docs.rs/ulua/latest/ulua/struct.Thread.html) along with enabling `feature = "async"` in `Cargo.toml`.

### Serde support

With the `serde` feature flag enabled, `ulua` allows you to serialize/deserialize any type that implements [`serde::Serialize`] and [`serde::Deserialize`] into/from [`ulua::Value`]. In addition, `ulua` provides the [`serde::Serialize`] trait implementation for `ulua::Value` (including `UserData` support).

[`serde::Serialize`]: https://docs.serde.rs/serde/ser/trait.Serialize.html
[`serde::Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
[`ulua::Value`]: https://docs.rs/ulua/latest/ulua/enum.Value.html

## Safety

One of `ulua`'s goals is to provide a *safe* API between Rust and Lua.
Every place where the Lua C API may trigger an error longjmp is protected by `lua_pcall`,
and the user of the library is protected from directly interacting with unsafe things like the Lua stack.
There is overhead associated with this safety.

Unfortunately, `ulua` does not provide absolute safety even without using `unsafe` .
This library contains a huge amount of unsafe code. There are almost certainly bugs still lurking in this library!
It is surprisingly, fiendishly difficult to use the Lua C API without the potential for unsafety.

## Panic handling

`ulua` wraps panics that are generated inside Rust callbacks in a regular Lua error. Panics can then be
resumed by returning or propagating the Lua error to Rust code.

For example:
``` rust
let lua = Lua::new();
let f = lua.create_function(|_, ()| -> LuaResult<()> {
    panic!("test panic");
})?;
lua.globals().set("rust_func", f)?;

let _ = lua.load(r#"
    local status, err = pcall(rust_func)
    print(err) -- prints: test panic
    error(err) -- propagate panic
"#).exec();

unreachable!()
```

Optionally, `ulua` can disable Rust panic catching in Lua via `pcall`/`xpcall` and automatically resume
them across the Lua API boundary. This is controlled via `LuaOptions` and done by wrapping the Lua `pcall`/`xpcall`
functions to prevent catching errors that are wrapped Rust panics.

`ulua` should also be panic safe in another way as well, which is that any `Lua` instances or handles
remain usable after a user generated panic, and such panics should not break internal invariants or
leak Lua stack space. This is mostly important to safely use `ulua` types in Drop impls, as you should not be
using panics for general error handling.

Below is a list of `ulua` behaviors that should be considered a bug.
If you encounter them, a bug report would be very welcome:

  + If you can cause UB with `ulua` without typing the word "unsafe", this is a bug.

  + If your program panics with a message that contains the string "ulua internal error", this is a bug.

  + Lua C API errors are handled by longjmp. All instances where the Lua C API would otherwise longjmp over calling stack frames should be guarded against, except in internal callbacks where this is intentional. If you detect that `ulua` is triggering a longjmp over your Rust stack frames, this is a bug!

  + If you detect that, after catching a panic or during a Drop triggered from a panic, a `Lua` or handle method is triggering other bugs or there is a Lua stack space leak, this is a bug. `ulua` instances are supposed to remain fully usable in the face of user generated panics. This guarantee does not extend to panics marked with "ulua internal error" simply because that is already indicative of a separate bug.

## Sandboxing

Please check the [Luau Sandboxing] page if you are interested in running untrusted Lua scripts in a controlled environment.

`ulua` provides the `Lua::sandbox` method for enabling sandbox mode (Luau only).

[Luau Sandboxing]: https://luau.org/sandbox

## License

This project is licensed under the [MIT license](LICENSE).
