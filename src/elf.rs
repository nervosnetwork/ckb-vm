// This module maps the data structure of different versions of goblin to the
// same internal structure.
use crate::machine::{VERSION0, VERSION1, VERSION2, VERSION3};
use crate::memory::{FLAG_EXECUTABLE, FLAG_FREEZED, round_page_down, round_page_up};
use crate::{Error, Register};
use bytes::Bytes;
use scroll::Pread;
use std::ops::Range;

// Even for different versions of goblin, their values must be consistent.
pub use goblin_v023::elf::program_header::{PF_R, PF_W, PF_X, PT_LOAD};
pub use goblin_v023::elf::section_header::SHF_EXECINSTR;

// GNU property note constants for RISC-V CFI features
// See: https://github.com/llvm/llvm-project/blob/c5aaee0bb07b221e5d3314bbdcf1abc4a604d6bd/llvm/include/llvm/BinaryFormat/ELF.h#L1809
#[allow(dead_code)]
const NT_GNU_PROPERTY_TYPE_0: u32 = 5;
// See: https://github.com/llvm/llvm-project/blob/c5aaee0bb07b221e5d3314bbdcf1abc4a604d6bd/llvm/include/llvm/BinaryFormat/ELF.h#L1845
const GNU_PROPERTY_RISCV_FEATURE_1_AND: u32 = 0xC000_0000;
// See: https://github.com/llvm/llvm-project/blob/c5aaee0bb07b221e5d3314bbdcf1abc4a604d6bd/llvm/include/llvm/BinaryFormat/ELF.h#L1911-L1915
const GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_UNLABELED: u32 = 1 << 0;
const GNU_PROPERTY_RISCV_FEATURE_1_CFI_SS: u32 = 1 << 1;
const GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_FUNC_SIG: u32 = 1 << 2;

/// Converts goblin's ELF flags into RISC-V flags
pub fn convert_flags(p_flags: u32, allow_freeze_writable: bool, vaddr: u64) -> Result<u8, Error> {
    let readable = p_flags & PF_R != 0;
    let writable = p_flags & PF_W != 0;
    let executable = p_flags & PF_X != 0;
    if !readable {
        return Err(Error::ElfSegmentUnreadable(vaddr));
    }
    if writable && executable {
        return Err(Error::ElfSegmentWritableAndExecutable(vaddr));
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CFI {
    pub lp_unlabeled: bool,
    pub ss: bool,
    pub lp_func_sig: bool,
}

impl CFI {
    pub fn allow_lpad(&self) -> bool {
        self.lp_unlabeled || self.lp_func_sig
    }
}

impl From<u8> for CFI {
    fn from(byte: u8) -> Self {
        Self {
            lp_unlabeled: byte & 0b0000_0001 != 0,
            ss: byte & 0b0000_0010 != 0,
            lp_func_sig: byte & 0b0000_0100 != 0,
        }
    }
}

impl From<CFI> for u8 {
    fn from(val: CFI) -> Self {
        (if val.lp_unlabeled { 0b0000_0001 } else { 0 })
            | (if val.ss { 0b0000_0010 } else { 0 })
            | (if val.lp_func_sig { 0b0000_0100 } else { 0 })
    }
}

#[derive(Default)]
pub struct ParseElfPortableData {
    pub entry: u64,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
    pub shstrtab_offset: usize,
}

impl ParseElfPortableData {
    pub fn from_v0<R: Register>(program: &Bytes) -> Result<Self, Error> {
        use goblin_v023::container::Ctx;
        use goblin_v023::elf::{Header, program_header::ProgramHeader as GoblinProgramHeader};
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
        let section_headers = vec![];
        Ok(Self {
            entry: header.e_entry,
            program_headers,
            section_headers,
            shstrtab_offset: header.e_shstrndx as usize,
        })
    }

    pub fn from_v1<R: Register>(program: &Bytes) -> Result<Self, Error> {
        use goblin_v040::container::Ctx;
        use goblin_v040::elf::{Header, program_header::ProgramHeader as GoblinProgramHeader};
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
        let section_headers = vec![];
        Ok(Self {
            entry: header.e_entry,
            program_headers,
            section_headers,
            shstrtab_offset: header.e_shstrndx as usize,
        })
    }

    pub fn from_v3<R: Register>(program: &Bytes) -> Result<Self, Error> {
        use goblin_v040::container::Ctx;
        use goblin_v040::elf::{
            Header, program_header::ProgramHeader as GoblinProgramHeader,
            section_header::SectionHeader as GoblinSectionHeader,
        };
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
        let section_headers = GoblinSectionHeader::parse(
            program,
            header.e_shoff as usize,
            header.e_shnum as usize,
            ctx,
        )?
        .iter()
        .map(SectionHeader::from_v1)
        .collect();
        Ok(Self {
            entry: header.e_entry,
            program_headers,
            section_headers,
            shstrtab_offset: header.e_shstrndx as usize,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramMetadata {
    pub actions: Vec<LoadingAction>,
    pub entry: u64,
    pub cfi: CFI,
}

/// Parse GNU property notes to extract RISC-V CFI feature flags.
fn parse_gnu_property_note(note_data: &[u8]) -> Result<CFI, Error> {
    let mut cfi = CFI::default();
    let mut buf = [0u8; 4];
    let mut offset: usize = 0;
    while offset + 8 <= note_data.len() {
        // Read property type (4 bytes) and property data size (4 bytes)
        buf.copy_from_slice(&note_data[offset..offset + 4]);
        let pr_type = u32::from_le_bytes(buf);
        buf.copy_from_slice(&note_data[offset + 4..offset + 8]);
        let pr_datasz = u32::from_le_bytes(buf) as usize;
        // Overflow or unreasonable size check.
        if pr_datasz > note_data.len() {
            return Err(Error::ElfParseError(
                "Unreasonable property data size".into(),
            ));
        }
        offset += 8;
        if pr_type == GNU_PROPERTY_RISCV_FEATURE_1_AND
            && pr_datasz >= 4
            && offset + 4 <= note_data.len()
        {
            buf.copy_from_slice(&note_data[offset..offset + 4]);
            let feature_flags = u32::from_le_bytes(buf);
            cfi.lp_unlabeled = feature_flags & GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_UNLABELED != 0;
            cfi.ss = feature_flags & GNU_PROPERTY_RISCV_FEATURE_1_CFI_SS != 0;
            cfi.lp_func_sig = feature_flags & GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_FUNC_SIG != 0;
        }
        // Align to 8 bytes for next property.
        let aligned_datasz = (pr_datasz + 7) & !7;
        offset += aligned_datasz;
    }
    Ok(cfi)
}

pub fn parse_elf<R: Register>(program: &Bytes, version: u32) -> Result<ProgramMetadata, Error> {
    // We did not use Elf::parse here to avoid triggering potential bugs in goblin.
    // * https://github.com/nervosnetwork/ckb-vm/issues/143
    let pepd = match version {
        VERSION0 => ParseElfPortableData::from_v0::<R>(program)?,
        VERSION1 | VERSION2 => ParseElfPortableData::from_v1::<R>(program)?,
        VERSION3 => ParseElfPortableData::from_v3::<R>(program)?,
        _ => ParseElfPortableData::from_v3::<R>(program)?,
    };
    let mut cfi = CFI::default();
    // CFI will only be parsed when using version 3. This avoids errors in older code from
    // versions 0 to 2 due to the newly added CFI parsing.
    if version >= VERSION3 {
        if pepd.shstrtab_offset >= pepd.section_headers.len() {
            return Err(Error::ElfParseError(
                "Invalid section header string table offset".into(),
            ));
        }
        let shstrtab = &pepd.section_headers[pepd.shstrtab_offset];
        let shstrtab_start = shstrtab.sh_offset as usize;
        let shstrtab_end = shstrtab.sh_offset.saturating_add(shstrtab.sh_size) as usize;
        if shstrtab_end > program.len() {
            return Err(Error::ElfParseError(
                "Section header string table exceeds program size".into(),
            ));
        }
        for section in &pepd.section_headers {
            // Get section name.
            let name_offset = shstrtab_start.saturating_add(section.sh_name);
            if name_offset >= shstrtab_end {
                return Err(Error::ElfParseError("Invalid section name offset".into()));
            }
            let name_bytes = &program[name_offset..shstrtab_end];
            let null_pos = name_bytes.iter().position(|&b| b == 0);
            if null_pos.is_none() {
                return Err(Error::ElfParseError(
                    "Section name is not null-terminated".into(),
                ));
            }
            let section_name = &name_bytes[..null_pos.unwrap()];
            // Look for .note.gnu.property section.
            if section_name != b".note.gnu.property" {
                continue;
            }
            if section.sh_size < 12 {
                return Err(Error::ElfParseError(
                    ".note.gnu.property section too small".into(),
                ));
            }
            let note_start = section.sh_offset as usize;
            let note_end = note_start.saturating_add(section.sh_size as usize);
            if note_end > program.len() {
                return Err(Error::ElfParseError(
                    ".note.gnu.property section exceeds program size".into(),
                ));
            }
            let note_data = &program[note_start..note_end];
            // Parse note header: namesz(4), descsz(4),                type(4),  name
            //                         4u32      16u32  NT_GNU_PROPERTY_TYPE_0  GNU\0
            let expect_note_header: [u8; 16] = [4, 0, 0, 0, 16, 0, 0, 0, 5, 0, 0, 0, 71, 78, 85, 0];
            if note_data.len() == 32 && note_data[..16] == expect_note_header {
                if let Ok(icfi) = parse_gnu_property_note(&note_data[16..]) {
                    cfi = icfi;
                }
            }
            break;
        }
    }

    let mut bytes: u64 = 0;
    let mut actions = vec![];
    for program_header in pepd.program_headers {
        if program_header.p_type == PT_LOAD {
            let aligned_start = round_page_down(program_header.p_vaddr);
            let padding_start = program_header.p_vaddr.wrapping_sub(aligned_start);
            let size = round_page_up(program_header.p_memsz.wrapping_add(padding_start));
            let slice_start = program_header.p_offset;
            let slice_end = program_header
                .p_offset
                .wrapping_add(program_header.p_filesz);
            if slice_start > slice_end || slice_end > program.len() as u64 {
                return Err(Error::ElfSegmentAddrOrSizeError(program_header.p_vaddr));
            }
            actions.push(LoadingAction {
                addr: aligned_start,
                size,
                flags: convert_flags(
                    program_header.p_flags,
                    version < VERSION1,
                    program_header.p_vaddr,
                )?,
                source: slice_start..slice_end,
                offset_from_addr: padding_start,
            });
            bytes = bytes.checked_add(slice_end - slice_start).ok_or_else(|| {
                Error::Unexpected(String::from("The bytes count overflowed on loading elf"))
            })?;
        }
    }
    Ok(ProgramMetadata {
        actions,
        entry: pepd.entry,
        cfi,
    })
}
