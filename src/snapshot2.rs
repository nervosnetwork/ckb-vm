use crate::{
    bits::roundup,
    elf::{LoadingAction, ProgramMetadata},
    machine::SupportMachine,
    memory::{Memory, FLAG_DIRTY},
    Error, Register, RISCV_GENERAL_REGISTER_NUMBER, RISCV_PAGESIZE,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::HashMap;

const PAGE_SIZE: u64 = RISCV_PAGESIZE as u64;

/// DataSource represents data source that can stay stable and possibly
/// immutable for the entire lifecycle duration of a VM instance. One example
/// can be the enclosing transaction when using CKB-VM in CKB's environment,
/// no matter where and when we run the CKB smart contract, the enclosing
/// transaction will always be the same down to every last byte. As a result,
/// we can leverage DataSource for snapshot optimizations: data that is already
/// locatable in the DataSource will not need to be included in the snapshot
/// again, all we need is an id to locate it, together with a pair of
/// offset / length to cut in to the correct slices.
pub trait DataSource<I: Clone + PartialEq> {
    fn load_data(&self, id: &I, offset: u64, length: u64) -> Result<Bytes, Error>;
}

#[derive(Clone, Debug)]
pub struct Snapshot2Context<I: Clone + PartialEq, D: DataSource<I>> {
    // page index -> (id, offset, flag)
    pages: HashMap<u64, (I, u64, u8)>,
    data_source: D,
}

impl<I: Clone + PartialEq, D: DataSource<I> + Default> Default for Snapshot2Context<I, D> {
    fn default() -> Self {
        Self::new(D::default())
    }
}

impl<I: Clone + PartialEq, D: DataSource<I>> Snapshot2Context<I, D> {
    pub fn new(data_source: D) -> Self {
        Self {
            pages: HashMap::default(),
            data_source,
        }
    }

    /// Resume a previously suspended machine from snapshot.
    pub fn resume<M: SupportMachine>(
        &mut self,
        machine: &mut M,
        snapshot: &Snapshot2<I>,
    ) -> Result<(), Error> {
        if machine.version() != snapshot.version {
            return Err(Error::InvalidVersion);
        }
        // A resume basically means we reside in a new context
        self.pages.clear();
        for (i, v) in snapshot.registers.iter().enumerate() {
            machine.set_register(i, M::REG::from_u64(*v));
        }
        machine.update_pc(M::REG::from_u64(snapshot.pc));
        machine.commit_pc();
        machine.set_cycles(snapshot.cycles);
        machine.set_max_cycles(snapshot.max_cycles);
        for (address, flag, id, offset, length) in &snapshot.pages_from_source {
            if address % PAGE_SIZE != 0 {
                return Err(Error::MemPageUnalignedAccess);
            }
            let data = self.data_source().load_data(id, *offset, *length)?;
            if data.len() as u64 % PAGE_SIZE != 0 {
                return Err(Error::MemPageUnalignedAccess);
            }
            machine.memory_mut().store_bytes(*address, &data)?;
            for i in 0..(data.len() as u64 / PAGE_SIZE) {
                let page = address / PAGE_SIZE + i;
                machine.memory_mut().set_flag(page, *flag)?;
            }
            self.track_pages(machine, *address, data.len() as u64, id, *offset)?;
        }
        for (address, flag, content) in &snapshot.dirty_pages {
            if address % PAGE_SIZE != 0 {
                return Err(Error::MemPageUnalignedAccess);
            }
            if content.len() as u64 % PAGE_SIZE != 0 {
                return Err(Error::MemPageUnalignedAccess);
            }
            machine.memory_mut().store_bytes(*address, content)?;
            for i in 0..(content.len() as u64 / PAGE_SIZE) {
                let page = address / PAGE_SIZE + i;
                machine.memory_mut().set_flag(page, *flag)?;
            }
        }
        Ok(())
    }

    pub fn data_source(&self) -> &D {
        &self.data_source
    }

    pub fn data_source_mut(&mut self) -> &mut D {
        &mut self.data_source
    }

    /// Similar to Memory::store_bytes, but this method also tracks memory
    /// pages whose entire content comes from DataSource
    pub fn store_bytes<M: SupportMachine>(
        &mut self,
        machine: &mut M,
        addr: u64,
        id: &I,
        offset: u64,
        length: u64,
    ) -> Result<(), Error> {
        let data = self.data_source.load_data(id, offset, length)?;
        machine.memory_mut().store_bytes(addr, &data)?;
        self.track_pages(machine, addr, data.len() as u64, id, offset)
    }

    /// Due to the design of ckb-vm right now, load_program function does not
    /// belong to SupportMachine yet. For Snapshot2Context to track memory pages
    /// from program in DataSource, we have to use the following steps now:
    ///
    /// 1. use elf::parse_elf to generate ProgramMetadata
    /// 2. use DefaultMachine::load_program_with_metadata to load the program
    /// 3. Pass ProgramMetadata to this method so we can track memory pages from
    /// program, so as to further reduce the size of the generated snapshot.
    ///
    /// One can also use the original DefaultMachine::load_program, and parse the
    /// ELF a second time to extract metadata for this method. However the above
    /// listed process saves us the time to parse the ELF again.
    pub fn mark_program<M: SupportMachine>(
        &mut self,
        machine: &mut M,
        metadata: &ProgramMetadata,
        id: &I,
        offset: u64,
    ) -> Result<(), Error> {
        for action in &metadata.actions {
            self.init_pages(machine, action, id, offset)?;
        }
        Ok(())
    }

    /// Create a snapshot for the passed machine.
    pub fn make_snapshot<M: SupportMachine>(&self, machine: &mut M) -> Result<Snapshot2<I>, Error> {
        let mut dirty_pages: Vec<(u64, u8, Vec<u8>)> = vec![];
        for i in 0..machine.memory().memory_pages() as u64 {
            if self.pages.contains_key(&i) {
                continue;
            }
            let flag = machine.memory_mut().fetch_flag(i)?;
            if flag & FLAG_DIRTY == 0 {
                continue;
            }
            let address = i * PAGE_SIZE;
            let mut data: Vec<u8> = machine.memory_mut().load_bytes(address, PAGE_SIZE)?.into();
            if let Some(last) = dirty_pages.last_mut() {
                if last.0 + last.2.len() as u64 == address && last.1 == flag {
                    last.2.append(&mut data);
                }
            }
            if !data.is_empty() {
                dirty_pages.push((address, flag, data));
            }
        }
        let mut pages_from_source: Vec<(u64, u8, I, u64, u64)> = vec![];
        let mut pages: Vec<u64> = self.pages.keys().copied().collect();
        pages.sort_unstable();
        for page in pages {
            let address = page * PAGE_SIZE;
            let (id, offset, flag) = &self.pages[&page];
            let mut appended_to_last = false;
            if let Some((last_address, last_flag, last_id, last_offset, last_length)) =
                pages_from_source.last_mut()
            {
                if *last_address + *last_length == address
                    && *last_flag == *flag
                    && *last_id == *id
                    && *last_offset + *last_length == *offset
                {
                    *last_length += PAGE_SIZE;
                    appended_to_last = true;
                }
            }
            if !appended_to_last {
                pages_from_source.push((address, *flag, id.clone(), *offset, PAGE_SIZE));
            }
        }
        let mut registers = [0u64; RISCV_GENERAL_REGISTER_NUMBER];
        for (i, v) in machine.registers().iter().enumerate() {
            registers[i] = v.to_u64();
        }
        Ok(Snapshot2 {
            pages_from_source,
            dirty_pages,
            version: machine.version(),
            registers,
            pc: machine.pc().to_u64(),
            cycles: machine.cycles(),
            max_cycles: machine.max_cycles(),
        })
    }

    fn init_pages<M: SupportMachine>(
        &mut self,
        machine: &mut M,
        action: &LoadingAction,
        id: &I,
        offset: u64,
    ) -> Result<(), Error> {
        let start = action.addr + action.offset_from_addr;
        let length = min(
            action.source.end - action.source.start,
            action.size - action.offset_from_addr,
        );
        self.track_pages(machine, start, length, id, offset + action.source.start)
    }

    fn track_pages<M: SupportMachine>(
        &mut self,
        machine: &mut M,
        start: u64,
        mut length: u64,
        id: &I,
        mut offset: u64,
    ) -> Result<(), Error> {
        let mut aligned_start = roundup(start, PAGE_SIZE);
        let aligned_bytes = aligned_start - start;
        if length < aligned_bytes {
            return Ok(());
        }
        offset += aligned_bytes;
        length -= aligned_bytes;
        while length >= PAGE_SIZE {
            let page = aligned_start / PAGE_SIZE;
            machine.memory_mut().clear_flag(page, FLAG_DIRTY)?;
            let flag = machine.memory_mut().fetch_flag(page)?;
            self.pages.insert(page, (id.clone(), offset, flag));
            aligned_start += PAGE_SIZE;
            length -= PAGE_SIZE;
            offset += PAGE_SIZE;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Snapshot2<I: Clone + PartialEq> {
    // (address, flag, id, source offset, source length)
    pub pages_from_source: Vec<(u64, u8, I, u64, u64)>,
    // (address, flag, content)
    pub dirty_pages: Vec<(u64, u8, Vec<u8>)>,
    pub version: u32,
    pub registers: [u64; RISCV_GENERAL_REGISTER_NUMBER],
    pub pc: u64,
    pub cycles: u64,
    pub max_cycles: u64,
}
