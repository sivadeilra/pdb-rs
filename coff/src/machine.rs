/*
pub const IMAGE_FILE_RELOCS_STRIPPED: Self = Self(0x0001); // Relocation info stripped from file.
pub const IMAGE_FILE_EXECUTABLE_IMAGE: Self = Self(0x0002); // File is executable  (i.e. no unresolved external references).
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: Self = Self(0x0004); // Line nunbers stripped from file.
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: Self = Self(0x0008); // Local symbols stripped from file.
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: Self = Self(0x0010); // Aggressively trim working set
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: Self = Self(0x0020); // App can handle >2gb addresses
pub const IMAGE_FILE_BYTES_REVERSED_LO: Self = Self(0x0080); // Bytes of machine word are reversed.
pub const IMAGE_FILE_32BIT_MACHINE: Self = Self(0x0100); // 32 bit word machine.
pub const IMAGE_FILE_DEBUG_STRIPPED: Self = Self(0x0200); // Debugging info stripped from file in .DBG file
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: Self = Self(0x0400); // If Image is on removable media, copy and run from the swap file.
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: Self = Self(0x0800); // If Image is on Net, copy and run from the swap file.
pub const IMAGE_FILE_SYSTEM: Self = Self(0x1000); // System File.
pub const IMAGE_FILE_DLL: Self = Self(0x2000); // File is a DLL.
pub const IMAGE_FILE_UP_SYSTEM_ONLY: Self = Self(0x4000); // File should only be run on a UP machine
pub const IMAGE_FILE_BYTES_REVERSED_HI: Self = Self(0x8000); // Bytes of machine word are reversed.
*/

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct IMAGE_FILE_MACHINE(pub u16);

impl IMAGE_FILE_MACHINE {
    pub const IMAGE_FILE_MACHINE_UNKNOWN: Self = Self(0);
    /// Useful for indicating we want to interact with the host and not a WoW guest.
    pub const IMAGE_FILE_MACHINE_TARGET_HOST: Self = Self(0x0001);
    /// Intel 386.
    pub const IMAGE_FILE_MACHINE_I386: Self = Self(0x014c);
    /// MIPS little-endian, 0x160 big-endian
    pub const IMAGE_FILE_MACHINE_R3000: Self = Self(0x0162);
    /// MIPS little-endian
    pub const IMAGE_FILE_MACHINE_R4000: Self = Self(0x0166);
    /// MIPS little-endian
    pub const IMAGE_FILE_MACHINE_R10000: Self = Self(0x0168);
    /// MIPS little-endian WCE v2
    pub const IMAGE_FILE_MACHINE_WCEMIPSV2: Self = Self(0x0169);
    /// Alpha_AXP
    pub const IMAGE_FILE_MACHINE_ALPHA: Self = Self(0x0184);
    /// SH3 little-endian
    pub const IMAGE_FILE_MACHINE_SH3: Self = Self(0x01a2);
    pub const IMAGE_FILE_MACHINE_SH3DSP: Self = Self(0x01a3);
    /// SH3E little-endian
    pub const IMAGE_FILE_MACHINE_SH3E: Self = Self(0x01a4);
    /// SH4 little-endian
    pub const IMAGE_FILE_MACHINE_SH4: Self = Self(0x01a6);
    /// SH5
    pub const IMAGE_FILE_MACHINE_SH5: Self = Self(0x01a8);
    /// ARM Little-Endian
    pub const IMAGE_FILE_MACHINE_ARM: Self = Self(0x01c0);
    /// ARM Thumb/Thumb-2 Little-Endian
    pub const IMAGE_FILE_MACHINE_THUMB: Self = Self(0x01c2);
    /// ARM Thumb-2 Little-Endian
    pub const IMAGE_FILE_MACHINE_ARMNT: Self = Self(0x01c4);
    pub const IMAGE_FILE_MACHINE_AM33: Self = Self(0x01d3);
    /// IBM PowerPC Little-Endian
    pub const IMAGE_FILE_MACHINE_POWERPC: Self = Self(0x01F0);
    pub const IMAGE_FILE_MACHINE_POWERPCFP: Self = Self(0x01f1);
    /// Intel 64
    pub const IMAGE_FILE_MACHINE_IA64: Self = Self(0x0200);
    /// MIPS
    pub const IMAGE_FILE_MACHINE_MIPS16: Self = Self(0x0266);
    /// ALPHA64
    pub const IMAGE_FILE_MACHINE_ALPHA64: Self = Self(0x0284);
    /// MIPS
    pub const IMAGE_FILE_MACHINE_MIPSFPU: Self = Self(0x0366);
    /// MIPS
    pub const IMAGE_FILE_MACHINE_MIPSFPU16: Self = Self(0x0466);
    pub const IMAGE_FILE_MACHINE_AXP64: Self = Self::IMAGE_FILE_MACHINE_ALPHA64;
    /// Infineon
    pub const IMAGE_FILE_MACHINE_TRICORE: Self = Self(0x0520);
    pub const IMAGE_FILE_MACHINE_CEF: Self = Self(0x0CEF);
    /// EFI Byte Code
    pub const IMAGE_FILE_MACHINE_EBC: Self = Self(0x0EBC);
    /// AMD64 (K8)
    pub const IMAGE_FILE_MACHINE_AMD64: Self = Self(0x8664);
    /// M32R little-endian
    pub const IMAGE_FILE_MACHINE_M32R: Self = Self(0x9041);
    /// ARM64 Little-Endian
    pub const IMAGE_FILE_MACHINE_ARM64: Self = Self(0xAA64);
    pub const IMAGE_FILE_MACHINE_CEE: Self = Self(0xC0EE);

    pub fn to_str_opt(self) -> Option<&'static str> {
        Some(match self.0 {
            0x0000 => "IMAGE_FILE_MACHINE_UNKNOWN",
            0x0001 => "IMAGE_FILE_MACHINE_TARGET_HOST",
            0x014c => "IMAGE_FILE_MACHINE_I386",
            0x0162 => "IMAGE_FILE_MACHINE_R3000",
            0x0166 => "IMAGE_FILE_MACHINE_R4000",
            0x0168 => "IMAGE_FILE_MACHINE_R10000",
            0x0169 => "IMAGE_FILE_MACHINE_WCEMIPSV2",
            0x0184 => "IMAGE_FILE_MACHINE_ALPHA",
            0x01a2 => "IMAGE_FILE_MACHINE_SH3",
            0x01a3 => "IMAGE_FILE_MACHINE_SH3DSP",
            0x01a4 => "IMAGE_FILE_MACHINE_SH3E",
            0x01a6 => "IMAGE_FILE_MACHINE_SH4",
            0x01a8 => "IMAGE_FILE_MACHINE_SH5",
            0x01c0 => "IMAGE_FILE_MACHINE_ARM",
            0x01c2 => "IMAGE_FILE_MACHINE_THUMB",
            0x01c4 => "IMAGE_FILE_MACHINE_ARMNT",
            0x01d3 => "IMAGE_FILE_MACHINE_AM33",
            0x01F0 => "IMAGE_FILE_MACHINE_POWERPC",
            0x01f1 => "IMAGE_FILE_MACHINE_POWERPCFP",
            0x0200 => "IMAGE_FILE_MACHINE_IA64",
            0x0266 => "IMAGE_FILE_MACHINE_MIPS16",
            0x0284 => "IMAGE_FILE_MACHINE_ALPHA64",
            0x0366 => "IMAGE_FILE_MACHINE_MIPSFPU",
            0x0466 => "IMAGE_FILE_MACHINE_MIPSFPU16",
            0x0520 => "IMAGE_FILE_MACHINE_TRICORE",
            0x0CEF => "IMAGE_FILE_MACHINE_CEF",
            0x0EBC => "IMAGE_FILE_MACHINE_EBC",
            0x8664 => "IMAGE_FILE_MACHINE_AMD64",
            0x9041 => "IMAGE_FILE_MACHINE_M32R",
            0xAA64 => "IMAGE_FILE_MACHINE_ARM64",
            0xC0EE => "IMAGE_FILE_MACHINE_CEE",
            _ => return None,
        })
    }

    pub fn to_str(self) -> &'static str {
        self.to_str_opt().unwrap_or("??")
    }
}

impl core::fmt::Debug for IMAGE_FILE_MACHINE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(s) = self.to_str_opt() {
            f.write_str(s)
        } else {
            write!(f, "??0x{:x}", self.0)
        }
    }
}
