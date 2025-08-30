use std::panic::catch_unwind;

use ulua::Lua;

fn main() {
    let lua = Lua::new();
    catch_unwind(|| lua.create_table().unwrap());
}
