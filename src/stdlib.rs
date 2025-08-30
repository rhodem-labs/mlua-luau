use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

/// Flags describing the set of lua standard libraries to load.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StdLib(u32);

impl StdLib {
    /// [`coroutine`](https://www.lua.org/manual/5.4/manual.html#6.2) library
    pub const COROUTINE: StdLib = StdLib(1);

    /// [`table`](https://www.lua.org/manual/5.4/manual.html#6.6) library
    pub const TABLE: StdLib = StdLib(1 << 1);

    /// [`os`](https://www.lua.org/manual/5.4/manual.html#6.9) library
    pub const OS: StdLib = StdLib(1 << 3);

    /// [`string`](https://www.lua.org/manual/5.4/manual.html#6.4) library
    pub const STRING: StdLib = StdLib(1 << 4);

    /// [`utf8`](https://www.lua.org/manual/5.4/manual.html#6.5) library
    pub const UTF8: StdLib = StdLib(1 << 5);

    /// [`bit`](https://www.lua.org/manual/5.2/manual.html#6.7) library
    pub const BIT: StdLib = StdLib(1 << 6);

    /// [`math`](https://www.lua.org/manual/5.4/manual.html#6.7) library
    pub const MATH: StdLib = StdLib(1 << 7);

    /// [`buffer`](https://luau.org/library#buffer-library) library
    pub const BUFFER: StdLib = StdLib(1 << 9);

    /// [`vector`](https://luau.org/library#vector-library) library
    pub const VECTOR: StdLib = StdLib(1 << 10);

    /// (**unsafe**) [`debug`](https://www.lua.org/manual/5.4/manual.html#6.10) library
    pub const DEBUG: StdLib = StdLib(1 << 31);

    /// No libraries
    pub const NONE: StdLib = StdLib(0);
    /// (**unsafe**) All standard libraries
    pub const ALL: StdLib = StdLib(u32::MAX);
    /// The safe subset of the standard libraries
    pub const ALL_SAFE: StdLib = StdLib(u32::MAX);

    pub fn contains(self, lib: Self) -> bool {
        (self & lib).0 != 0
    }
}

impl BitAnd for StdLib {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        StdLib(self.0 & rhs.0)
    }
}

impl BitAndAssign for StdLib {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = StdLib(self.0 & rhs.0)
    }
}

impl BitOr for StdLib {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        StdLib(self.0 | rhs.0)
    }
}

impl BitOrAssign for StdLib {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = StdLib(self.0 | rhs.0)
    }
}

impl BitXor for StdLib {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        StdLib(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for StdLib {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = StdLib(self.0 ^ rhs.0)
    }
}
