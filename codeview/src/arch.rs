//! Module tree for architecture-specific definitions

macro_rules! register_set {
    (
        $( #[$a:meta] )*
        enum $ty_name:ident {
            $(
                $reg_name:ident = $reg_value:expr,
            )*
        }
    ) => {

        $( #[$a] )*
        #[allow(missing_docs)]

        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $ty_name(pub u16);

        #[allow(missing_docs)]
        impl $ty_name {
            $(
                pub const $reg_name: $ty_name = $ty_name($reg_value);
            )*

            #[inline(never)]
            pub fn get_name(self) -> Option<&'static str> {
                match self {
                    $(
                        Self::$reg_name => Some(stringify!($reg_name)),
                    )*
                    _ => None,
                }
            }

            #[inline(never)]
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $(
                        stringify!($reg_name) => Some(Self::$reg_name),
                    )*
                    _ => None,
                }
            }
        }

        impl core::fmt::Debug for $ty_name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                if let Some(s) = self.get_name() {
                    f.write_str(s)
                } else {
                    write!(f, "??(0x{:x})", self.0)
                }
            }
        }
    }
}

/// Identifies COFF CPU architectures.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Arch {
    /// AMD64
    AMD64,
    /// ARM64, including ARM64EC, ARM64X
    ARM64,
    /// X86
    X86,
}

pub mod amd64;
pub mod arm64;
pub mod x86;

/// Identifies a register in a specific architecture
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ArchReg {
    /// AMD64
    Amd64(amd64::Amd64Reg),
    /// X86
    X86(x86::X86Reg),
    /// ARM64
    Arm64(arm64::Arm64Reg),
}

impl ArchReg {
    /// Ties an arch and a reg
    pub fn from_arch_reg(arch: Arch, reg: u16) -> Self {
        match arch {
            Arch::AMD64 => Self::Amd64(amd64::Amd64Reg(reg)),
            Arch::ARM64 => Self::Arm64(arm64::Arm64Reg(reg)),
            Arch::X86 => Self::X86(x86::X86Reg(reg)),
        }
    }
}

use core::fmt::{Debug, Display};

impl Debug for ArchReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchReg::Amd64(reg) => reg.fmt(f),
            ArchReg::X86(reg) => reg.fmt(f),
            ArchReg::Arm64(reg) => reg.fmt(f),
        }
    }
}

impl Display for ArchReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchReg::Amd64(reg) => reg.fmt(f),
            ArchReg::X86(reg) => reg.fmt(f),
            ArchReg::Arm64(reg) => reg.fmt(f),
        }
    }
}
