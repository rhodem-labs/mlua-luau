use std::sync::Arc;

use ulua::{Error, GCMode, Lua, Result, UserData};

#[test]
fn test_memory_limit() -> Result<()> {
    let lua = Lua::new();

    let initial_memory = lua.used_memory();
    assert!(
        initial_memory > 0,
        "used_memory reporting is wrong, lua uses memory for stdlib"
    );

    let f = lua
        .load("local t = {}; for i = 1,10000 do t[i] = i end")
        .into_function()?;
    f.call::<()>(()).expect("should trigger no memory limit");

    lua.set_memory_limit(initial_memory + 10000)?;
    match f.call::<()>(()) {
        Err(Error::MemoryError(_)) => {}
        something_else => panic!("did not trigger memory error: {:?}", something_else),
    };

    lua.set_memory_limit(0)?;
    f.call::<()>(()).expect("should trigger no memory limit");

    // Test memory limit during chunk loading
    lua.set_memory_limit(1024)?;
    match lua
        .load("local t = {}; for i = 1,10000 do t[i] = i end")
        .into_function()
    {
        Err(Error::MemoryError(_)) => {}
        _ => panic!("did not trigger memory error"),
    };

    Ok(())
}

#[test]
fn test_memory_limit_thread() -> Result<()> {
    let lua = Lua::new();

    let f = lua
        .load("local t = {}; for i = 1,10000 do t[i] = i end")
        .into_function()?;

    let thread = lua.create_thread(f)?;
    lua.set_memory_limit(lua.used_memory() + 10000)?;
    match thread.resume::<()>(()) {
        Err(Error::MemoryError(_)) => {}
        something_else => panic!("did not trigger memory error: {:?}", something_else),
    };

    Ok(())
}

#[test]
fn test_gc_control() -> Result<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    assert!(lua.gc_is_running());
    lua.gc_stop();
    assert!(!lua.gc_is_running());
    lua.gc_restart();
    assert!(lua.gc_is_running());

    assert_eq!(lua.gc_inc(200, 100, 13), GCMode::Incremental);

    struct MyUserdata(#[allow(unused)] Arc<()>);
    impl UserData for MyUserdata {}

    let rc = Arc::new(());
    globals.set("userdata", lua.create_userdata(MyUserdata(rc.clone()))?)?;
    globals.raw_remove("userdata")?;

    assert_eq!(Arc::strong_count(&rc), 2);
    lua.gc_collect()?;
    lua.gc_collect()?;
    assert_eq!(Arc::strong_count(&rc), 1);

    Ok(())
}