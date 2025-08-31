use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;

use crate::chunk::ChunkMode;
use crate::error::Result;
use crate::function::Function;
use crate::state::{callback_error_ext, ExtraData, Lua};
use crate::traits::{FromLuaMulti, IntoLua};
use crate::types::MaybeSend;

pub use require::{NavigateError, Require, TextRequirer};

// Since Luau has some missing standard functions, we re-implement them here

impl Lua {
    /// Create a custom Luau `require` function using provided [`Require`] implementation to find
    /// and load modules.
    pub fn create_require_function<R: Require + MaybeSend + 'static>(&self, require: R) -> Result<Function> {
        require::create_require_function(self, require)
    }

    pub(crate) unsafe fn configure_luau(&self) -> Result<()> {
        let globals = self.globals();

        globals.raw_set("loadstring", self.create_c_function(lua_loadstring)?)?;

        // Set `_VERSION` global to include version number
        // The environment variable `LUAU_VERSION` set by the build script
        if let Some(version) = ffi::luau_version() {
            globals.raw_set("_VERSION", format!("Luau {version}"))?;
        }

        // Enable default `require` implementation
        let require = self.create_require_function(require::TextRequirer::new())?;
        self.globals().raw_set("require", require)?;

        Ok(())
    }
}

unsafe extern "C-unwind" fn lua_loadstring(state: *mut ffi::lua_State) -> c_int {
    callback_error_ext(state, ptr::null_mut(), false, move |extra, nargs| {
        let rawlua = (*extra).raw_lua();

        let (chunk, chunk_name) =
            <(String, Option<String>)>::from_stack_args(nargs, 1, Some("loadstring"), rawlua, state)?;
        let chunk_name = chunk_name.as_deref().unwrap_or("=(loadstring)");
        (rawlua.lua())
            .load(chunk)
            .set_name(chunk_name)
            .set_mode(ChunkMode::Text)
            .into_function()?
            .push_into_stack(rawlua, state)?;
        Ok(1)
    })
}

mod require;
