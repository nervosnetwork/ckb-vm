#![no_main]
use ckb_vm::{
    elf::{LoadingAction, ProgramMetadata},
    machine::VERSION2,
    memory::{round_page_down, round_page_up, FLAG_EXECUTABLE, FLAG_FREEZED},
    snapshot2::{DataSource, Snapshot2Context},
    Bytes, CoreMachine, DefaultMachine, DefaultMachineBuilder, Memory, DEFAULT_MEMORY_SIZE, ISA_A,
    ISA_B, ISA_IMC, ISA_MOP, RISCV_PAGESIZE,
};
use ckb_vm_definitions::asm::AsmCoreMachine;
use libfuzzer_sys::fuzz_target;
use std::collections::VecDeque;

struct Deque {
    n: VecDeque<u8>,
}

impl Deque {
    fn new(data: [u8; 96]) -> Self {
        Self {
            n: VecDeque::from(data),
        }
    }

    fn u8(&mut self) -> u8 {
        let r = self.n.pop_front().unwrap();
        self.n.push_back(r);
        r
    }

    fn u32(&mut self) -> u32 {
        let mut r = [0u8; 4];
        r.fill_with(|| self.u8());
        u32::from_le_bytes(r)
    }
}

const DATA_SOURCE_PROGRAM: u32 = 0;
const DATA_SOURCE_CONTENT: u32 = 1;
const MAX_TX_SIZE: u32 = 600_000;

#[derive(Clone)]
pub struct DummyData {
    pub program: Bytes,
    pub content: Bytes,
}

impl DataSource<u32> for DummyData {
    fn load_data(&self, id: &u32, offset: u64, length: u64) -> Option<(Bytes, u64)> {
        let data = match *id {
            DATA_SOURCE_PROGRAM => &self.program,
            DATA_SOURCE_CONTENT => &self.content,
            _ => unreachable!(),
        };
        let offset = std::cmp::min(offset as usize, data.len());
        let full_size = data.len() - offset;
        let real_size = if length > 0 {
            std::cmp::min(full_size, length as usize)
        } else {
            full_size
        };
        Some((data.slice(offset..offset + real_size), full_size as u64))
    }
}

fn build_machine() -> DefaultMachine<Box<AsmCoreMachine>> {
    let isa = ISA_IMC | ISA_A | ISA_B | ISA_MOP;
    let core_machine = AsmCoreMachine::new(isa, VERSION2, u64::max_value());
    DefaultMachineBuilder::new(core_machine).build()
}

fuzz_target!(|data: [u8; 96]| {
    let mut deque = Deque::new(data);
    let dummy_data = {
        let mut program = vec![0u8; (deque.u32() % MAX_TX_SIZE) as usize];
        for i in 0..program.len() {
            program[i] = (i % 3) as u8;
        }
        let mut content = vec![0u8; (deque.u32() % MAX_TX_SIZE) as usize];
        for i in 0..content.len() {
            content[i] = (i % 5) as u8 + 10;
        }
        DummyData {
            program: program.into(),
            content: content.into(),
        }
    };
    let mut loading_action_vec: Vec<LoadingAction> = Vec::new();
    for _ in 0..2 {
        let p_vaddr = deque.u32() as u64;
        let p_memsz = deque.u32() as u64;
        let p_offset = deque.u32() as u64;
        let p_filesz = deque.u32() as u64;
        let aligned_start = round_page_down(p_vaddr);
        let padding_start = (p_vaddr).wrapping_sub(aligned_start);
        let size = round_page_up((p_memsz).wrapping_add(padding_start));
        let slice_start = p_offset;
        let slice_end = p_offset.wrapping_add(p_filesz);
        if slice_start >= slice_end || slice_end >= dummy_data.program.len() as u64 {
            return;
        }
        loading_action_vec.push(LoadingAction {
            addr: aligned_start,
            size,
            flags: FLAG_EXECUTABLE | FLAG_FREEZED,
            source: slice_start as u64..slice_end as u64,
            offset_from_addr: padding_start,
        })
    }
    let mut ctx = Snapshot2Context::new(dummy_data.clone());
    let metadata = ProgramMetadata {
        actions: loading_action_vec.clone(),
        entry: 0,
    };

    let mut machine1 = build_machine();
    let mut machine2 = build_machine();
    let result = machine1.load_program_with_metadata(&dummy_data.program, &metadata, &vec![]);
    if result.is_err() {
        return;
    }
    let result = ctx.mark_program(&mut machine1, &metadata, &0, 0);
    if result.is_err() {
        return;
    }
    for _ in 0..2 {
        let length = deque.u32() as u64;
        let offset = deque.u32() as u64;
        let addr = deque.u32() as u64;
        let result = ctx.store_bytes(&mut machine1, addr, &DATA_SOURCE_CONTENT, offset, length, 0);
        if result.is_err() {
            return;
        }
    }
    for _ in 0..2 {
        let length = deque.u32() as u64;
        let offset = deque.u32() as u64;
        let addr = deque.u32() as u64;
        let data = dummy_data
            .load_data(&DATA_SOURCE_CONTENT, offset, length)
            .unwrap()
            .0;
        let result = machine1.memory_mut().store_bytes(addr, &data);
        if result.is_err() {
            continue;
        }
    }
    let snapshot = ctx.make_snapshot(&mut machine1).unwrap();
    ctx.resume(&mut machine2, &snapshot).unwrap();
    for i in 0..DEFAULT_MEMORY_SIZE / RISCV_PAGESIZE {
        let mem1 = machine1
            .memory_mut()
            .load_bytes((i * RISCV_PAGESIZE) as u64, RISCV_PAGESIZE as u64)
            .unwrap();
        let mem2 = machine2
            .memory_mut()
            .load_bytes((i * RISCV_PAGESIZE) as u64, RISCV_PAGESIZE as u64)
            .unwrap();
        assert_eq!(mem1, mem2);
        let flag1 = machine1.memory_mut().fetch_flag(i as u64);
        let flag2 = machine2.memory_mut().fetch_flag(i as u64);
        assert_eq!(flag1, flag2);
    }
});
