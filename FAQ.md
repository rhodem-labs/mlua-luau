# ulua FAQ

This file is for general questions that don't fit into the README or crate docs.

## Loading a C module fails with error `undefined symbol: lua_xxx`. How to fix?

Add the following rustflags to your [.cargo/config](http://doc.crates.io/config.html) in order to properly export Lua symbols:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-args=-rdynamic"]

[target.x86_64-apple-darwin]
rustflags = ["-C", "link-args=-rdynamic"]
```