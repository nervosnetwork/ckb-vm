// This module maps the data structure of different versions of goblin to the
// same internal structure.
use crate::memory::{FLAG_EXECUTABLE, FLAG_FREEZED};
use crate::Error;

// Even for different versions of goblin, their values must be consistent.
pub use goblin_v023::elf::program_header::{PF_R, PF_W, PF_X, PT_LOAD};
pub use goblin_v023::elf::section_header::SHF_EXECINSTR;

/// Converts goblin's ELF flags into RISC-V flags
pub fn convert_flags(p_flags: u32, allow_freeze_writable: bool) -> Result<u8, Error> {
    let readable = p_flags & PF_R != 0;
    let writable = p_flags & PF_W != 0;
    let executable = p_flags & PF_X != 0;
    if (!readable) || (writable && executable) {
        return Err(Error::InvalidPermission);
    }
    if executable {
        Ok(FLAG_EXECUTABLE | FLAG_FREEZED)
    } else if writable && !allow_freeze_writable {
        Ok(0)
    } else {
        Ok(FLAG_FREEZED)
    }
}

/// Same as goblin::elf::ProgramHeader.
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl ProgramHeader {
    pub fn from_v0(header: &goblin_v023::elf::ProgramHeader) -> Self {
        Self {
            p_type: header.p_type,
            p_flags: header.p_flags,
            p_offset: header.p_offset,
            p_vaddr: header.p_vaddr,
            p_paddr: header.p_paddr,
            p_filesz: header.p_filesz,
            p_memsz: header.p_memsz,
            p_align: header.p_align,
        }
    }

    pub fn from_v1(header: &goblin_v034::elf::ProgramHeader) -> Self {
        Self {
            p_type: header.p_type,
            p_flags: header.p_flags,
            p_offset: header.p_offset,
            p_vaddr: header.p_vaddr,
            p_paddr: header.p_paddr,
            p_filesz: header.p_filesz,
            p_memsz: header.p_memsz,
            p_align: header.p_align,
        }
    }
}

/// Same as goblin::elf::SectionHeader.
pub struct SectionHeader {
    pub sh_name: usize,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl SectionHeader {
    pub fn from_v0(header: &goblin_v023::elf::SectionHeader) -> Self {
        Self {
            sh_name: header.sh_name,
            sh_type: header.sh_type,
            sh_flags: header.sh_flags,
            sh_addr: header.sh_addr,
            sh_offset: header.sh_offset,
            sh_size: header.sh_size,
            sh_link: header.sh_link,
            sh_info: header.sh_info,
            sh_addralign: header.sh_addralign,
            sh_entsize: header.sh_entsize,
        }
    }

    pub fn from_v1(header: &goblin_v034::elf::SectionHeader) -> Self {
        Self {
            sh_name: header.sh_name,
            sh_type: header.sh_type,
            sh_flags: header.sh_flags,
            sh_addr: header.sh_addr,
            sh_offset: header.sh_offset,
            sh_size: header.sh_size,
            sh_link: header.sh_link,
            sh_info: header.sh_info,
            sh_addralign: header.sh_addralign,
            sh_entsize: header.sh_entsize,
        }
    }
}
