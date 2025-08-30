use std::fmt;

#[cfg(feature = "serde")]
use serde::ser::{Serialize, SerializeTupleStruct, Serializer};

/// A Luau vector type.
///
/// By default vectors are 3-dimensional, but can be 4-dimensional
/// if the `vector4` feature is enabled.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector(pub(crate) [f32; Self::SIZE]);

impl fmt::Display for Vector {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "vector4"))]
        return write!(f, "vector({}, {}, {})", self.x(), self.y(), self.z());
        #[cfg(feature = "vector4")]
        return write!(f, "vector({}, {}, {}, {})", self.x(), self.y(), self.z(), self.w());
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vector {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut ts = serializer.serialize_tuple_struct("Vector", Self::SIZE)?;
        ts.serialize_field(&self.x())?;
        ts.serialize_field(&self.y())?;
        ts.serialize_field(&self.z())?;
        #[cfg(feature = "vector4")]
        ts.serialize_field(&self.w())?;
        ts.end()
    }
}

impl PartialEq<[f32; Self::SIZE]> for Vector {
    #[inline]
    fn eq(&self, other: &[f32; Self::SIZE]) -> bool {
        self.0 == *other
    }
}

impl crate::types::LuaType for Vector {
    const TYPE_ID: std::os::raw::c_int = ffi::LUA_TVECTOR;
}
