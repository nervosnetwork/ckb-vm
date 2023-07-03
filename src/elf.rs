// This module maps the data structure of different versions of goblin to the
// same internal structure.
use crate::machine::VERSION1;
use crate::memory::{round_page_down, round_page_up, FLAG_EXECUTABLE, FLAG_FREEZED};
use crate::{Error, Register};
use bytes::Bytes;
use scroll::Pread;
use std::ops::Range;

// Even for different versions of goblin, their values must be consistent.
pub use goblin_v023::elf::program_header::{PF_R, PF_W, PF_X, PT_LOAD};
pub use goblin_v023::elf::section_header::SHF_EXECINSTR;

/// Converts goblin's ELF flags into RISC-V flags
pub fn convert_flags(p_flags: u32, allow_freeze_writable: bool) -> Result<u8, Error> {
    let readable = p_flags & PF_R != 0;
    let writable = p_flags & PF_W != 0;
    let executable = p_flags & PF_X != 0;
    if !readable {
        return Err(Error::ElfSegmentUnreadable);
    }
    if writable && executable {
        return Err(Error::ElfSegmentWritableAndExecutable);
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

    pub fn from_v1(header: &goblin_v040::elf::ProgramHeader) -> Self {
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

    pub fn from_v1(header: &goblin_v040::elf::SectionHeader) -> Self {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadingAction {
    pub addr: u64,
    pub size: u64,
    pub flags: u8,
    pub source: Range<u64>,
    pub offset_from_addr: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramMetadata {
    pub actions: Vec<LoadingAction>,
    pub entry: u64,
}

pub fn parse_elf<R: Register>(program: &Bytes, version: u32) -> Result<ProgramMetadata, Error> {
    // We did not use Elf::parse here to avoid triggering potential bugs in goblin.
    // * https://github.com/nervosnetwork/ckb-vm/issues/143
    let (entry, program_headers): (u64, Vec<ProgramHeader>) = if version < VERSION1 {
        use goblin_v023::container::Ctx;
        use goblin_v023::elf::{program_header::ProgramHeader as GoblinProgramHeader, Header};
        let header = program.pread::<Header>(0)?;
        let container = header.container().map_err(|_e| Error::ElfBits)?;
        let endianness = header.endianness().map_err(|_e| Error::ElfBits)?;
        if R::BITS != if container.is_big() { 64 } else { 32 } {
            return Err(Error::ElfBits);
        }
        let ctx = Ctx::new(container, endianness);
        let program_headers = GoblinProgramHeader::parse(
            program,
            header.e_phoff as usize,
            header.e_phnum as usize,
            ctx,
        )?
        .iter()
        .map(ProgramHeader::from_v0)
        .collect();
        (header.e_entry, program_headers)
    } else {
        use goblin_v040::container::Ctx;
        use goblin_v040::elf::{program_header::ProgramHeader as GoblinProgramHeader, Header};
        let header = program.pread::<Header>(0)?;
        let container = header.container().map_err(|_e| Error::ElfBits)?;
        let endianness = header.endianness().map_err(|_e| Error::ElfBits)?;
        if R::BITS != if container.is_big() { 64 } else { 32 } {
            return Err(Error::ElfBits);
        }
        let ctx = Ctx::new(container, endianness);
        let program_headers = GoblinProgramHeader::parse(
            program,
            header.e_phoff as usize,
            header.e_phnum as usize,
            ctx,
        )?
        .iter()
        .map(ProgramHeader::from_v1)
        .collect();
        (header.e_entry, program_headers)
    };
    let mut bytes: u64 = 0;
    let mut actions = vec![];
    for program_header in program_headers {
        if program_header.p_type == PT_LOAD {
            let aligned_start = round_page_down(program_header.p_vaddr);
            let padding_start = program_header.p_vaddr.wrapping_sub(aligned_start);
            let size = round_page_up(program_header.p_memsz.wrapping_add(padding_start));
            let slice_start = program_header.p_offset;
            let slice_end = program_header
                .p_offset
                .wrapping_add(program_header.p_filesz);
            if slice_start > slice_end || slice_end > program.len() as u64 {
                return Err(Error::ElfSegmentAddrOrSizeError);
            }
            actions.push(LoadingAction {
                addr: aligned_start,
                size,
                flags: convert_flags(program_header.p_flags, version < VERSION1)?,
                source: slice_start..slice_end,
                offset_from_addr: padding_start,
            });
            bytes = bytes.checked_add(slice_end - slice_start).ok_or_else(|| {
                Error::Unexpected(String::from("The bytes count overflowed on loading elf"))
            })?;
        }
    }
    Ok(ProgramMetadata { actions, entry })
}
