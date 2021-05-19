use crate::instructions::Register;
use crate::memory::Memory;
use crate::memory::FLAG_DIRTY;
use crate::{
    CoreMachine, Error, RISCV_GENERAL_REGISTER_NUMBER, RISCV_PAGES, RISCV_PAGESIZE,
    RISCV_PAGE_SHIFTS,
};
use serde::{Deserialize, Serialize};

// Snapshot provides a mechanism for suspending and resuming a virtual machine.
//
// When cycle limit is too low, our work won't be finished when we hit it and
// a "max cycles exceeded" error will be returned. At this time, a snapshot could
// be created, we can create a new virtual machine from this snapshot, provide
// more cycles and then continue to run it.
//
// For the following data, we simply save them, and then restore them.
//   - machine.version
//   - machine.pc
//   - machine.registers
//
// For memory, the situation becomes more complicated. Every memory page has
// page flag where each page flag stores a optional FLAG_DIRTY. When this page
// is wrote, the memory instance will set this bit to 1, this helps us keep
// track of those pages that have been modified. After `machine.load_elf`, We
// clean up all dirty flags, so after the program terminates, all pages marked
// as dirty are the pages that have been modified by the program. We only store
// these pages in the snapshot.

#[derive(Default, Deserialize, Serialize)]
pub struct Snapshot {
    pub version: u32,
    pub registers: [u64; RISCV_GENERAL_REGISTER_NUMBER],
    pub pc: u64,
    pub page_indices: Vec<u64>,
    pub pages: Vec<Vec<u8>>,
}

pub fn make_snapshot<T: CoreMachine>(machine: &mut T) -> Result<Snapshot, Error> {
    let mut snap = Snapshot {
        version: machine.version(),
        pc: machine.pc().to_u64(),
        ..Default::default()
    };
    for (i, v) in machine.registers().iter().enumerate() {
        snap.registers[i] = v.to_u64();
    }

    for i in 0..RISCV_PAGES {
        let flag = machine.memory_mut().fetch_flag(i as u64)?;
        if flag & FLAG_DIRTY != 0 {
            let addr_from = i << RISCV_PAGE_SHIFTS;
            let addr_to = (i + 1) << RISCV_PAGE_SHIFTS;

            let mut page = vec![0; RISCV_PAGESIZE];
            for i in (addr_from..addr_to).step_by(8) {
                let v64 = machine
                    .memory_mut()
                    .load64(&T::REG::from_u64(i as u64))?
                    .to_u64();
                let j = i - addr_from;
                page[j] = v64 as u8;
                page[j + 1] = (v64 >> 8) as u8;
                page[j + 2] = (v64 >> 16) as u8;
                page[j + 3] = (v64 >> 24) as u8;
                page[j + 4] = (v64 >> 32) as u8;
                page[j + 5] = (v64 >> 40) as u8;
                page[j + 6] = (v64 >> 48) as u8;
                page[j + 7] = (v64 >> 56) as u8;
            }

            snap.page_indices.push(i as u64);
            snap.pages.push(page);
        }
    }
    Ok(snap)
}

pub fn resume<T: CoreMachine>(machine: &mut T, snapshot: &Snapshot) -> Result<(), Error> {
    if machine.version() != snapshot.version {
        return Err(Error::InvalidVersion);
    }
    for (i, v) in snapshot.registers.iter().enumerate() {
        machine.set_register(i, T::REG::from_u64(*v));
    }
    machine.update_pc(T::REG::from_u64(snapshot.pc));
    machine.commit_pc();
    for i in 0..snapshot.page_indices.len() {
        let page_index = snapshot.page_indices[i];
        let page = &snapshot.pages[i];
        let addr_from = page_index << RISCV_PAGE_SHIFTS;
        machine.memory_mut().store_bytes(addr_from, &page[..])?;
    }

    Ok(())
}
