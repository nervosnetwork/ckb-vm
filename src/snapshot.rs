use crate::instructions::Register;
use crate::memory::Memory;
use crate::memory::FLAG_DIRTY;
use crate::{
    CoreMachine, Error, RISCV_GENERAL_REGISTER_NUMBER, RISCV_PAGES, RISCV_PAGESIZE,
    RISCV_PAGE_SHIFTS,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Snapshot {
    pub version: u32,
    pub registers: [u64; RISCV_GENERAL_REGISTER_NUMBER],
    pub pc: u64,
    pub page_indices: Vec<u64>,
    pub pages: Vec<Vec<u8>>,
}

pub fn make_snapshot<T: CoreMachine>(machine: &mut T) -> Result<Snapshot, Error> {
    let mut snap = Snapshot::default();
    snap.version = machine.version();
    for (i, v) in machine.registers().iter().enumerate() {
        snap.registers[i] = v.to_u64();
    }
    snap.pc = machine.pc().to_u64();

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
    machine.set_pc(T::REG::from_u64(snapshot.pc));
    for i in 0..snapshot.page_indices.len() {
        let page_index = snapshot.page_indices[i];
        let page = &snapshot.pages[i];
        let addr_from = page_index << RISCV_PAGE_SHIFTS;
        machine
            .memory_mut()
            .store_bytes(addr_from, &page[..])
            .unwrap();
    }

    Ok(())
}
