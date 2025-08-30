use std::io;

#[cfg(feature = "serde")]
use serde::ser::{Serialize, Serializer};

use crate::state::RawLua;
use crate::types::ValueRef;

/// A Luau buffer type.
///
/// See the buffer [documentation] for more information.
///
/// [documentation]: https://luau.org/library#buffer-library
#[derive(Clone, Debug, PartialEq)]
pub struct Buffer(pub(crate) ValueRef);

struct BufferCursor(Buffer, usize);

impl io::Read for BufferCursor {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let lua = self.0 .0.lua.lock();
        let data = self.0.as_slice(&lua);
        if self.1 == data.len() {
            return Ok(0);
        }
        let len = buf.len().min(data.len() - self.1);
        buf[..len].copy_from_slice(&data[self.1..self.1 + len]);
        self.1 += len;
        Ok(len)
    }
}

impl io::Write for BufferCursor {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let lua = self.0 .0.lua.lock();
        let data = self.0.as_slice_mut(&lua);
        if self.1 == data.len() {
            return Ok(0);
        }
        let len = buf.len().min(data.len() - self.1);
        data[self.1..self.1 + len].copy_from_slice(&buf[..len]);
        self.1 += len;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Seek for BufferCursor {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let lua = self.0 .0.lua.lock();
        let data = self.0.as_slice(&lua);
        let new_offset = match pos {
            io::SeekFrom::Start(offset) => offset as i64,
            io::SeekFrom::End(offset) => data.len() as i64 + offset,
            io::SeekFrom::Current(offset) => self.1 as i64 + offset,
        };
        if new_offset < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid seek to a negative position",
            ));
        }
        if new_offset as usize > data.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid seek to a position beyond the end of the buffer",
            ));
        }
        self.1 = new_offset as usize;
        Ok(self.1 as u64)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Buffer {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let lua = self.0.lua.lock();
        serializer.serialize_bytes(self.as_slice(&lua))
    }
}

impl crate::types::LuaType for Buffer {
    const TYPE_ID: std::os::raw::c_int = ffi::LUA_TBUFFER;
}
