use super::{emitter::Emitter, instructions::is_jitable_instruction, tracer::Tracer, value::Value};
use crate::{
    decoder::build_imac_decoder,
    instructions::{execute, instruction_length, is_basic_block_end_instruction},
    CoreMachine, DefaultMachineBuilder, Error, InstructionCycleFunc, Machine, Memory, Register,
    SparseMemory, SupportMachine, Syscalls,
};
use bytes::Bytes;
use fnv::FnvHashMap;
use libc::{c_int, uint64_t};
use memmap::{Mmap, MmapMut};
use std::cmp::Ordering;
use std::mem;
use std::pin::Pin;
use std::ptr;
use std::rc::Rc;

// In this module, we attach PC register to general purpose register array
// for unified processing
pub const REGISTER_PC: usize = 32;
const ASM_DATA_REGISTERS_SLOTS: usize = 33;

const JIT_SEGMENT_LENGTH: usize = 1024 * 1024;

// This is the interface used across Rust side and C side to pass machine
// related data.
#[repr(C)]
struct AsmData {
    registers: [uint64_t; ASM_DATA_REGISTERS_SLOTS],
    rust_data: ptr::NonNull<RustData>,
}

impl AsmData {
    fn new(rust_data: &RustData) -> Self {
        AsmData {
            registers: [0; ASM_DATA_REGISTERS_SLOTS],
            rust_data: ptr::NonNull::from(rust_data),
        }
    }
}

// This holds all the Rust data in JitCoreMachine, it's organized this way
// so we can pass the whole Rust related struct as void* in C side for ease
// of FFI handling.
struct RustData {
    // Machine related data, should be cleared before each run
    memory: SparseMemory<u64>,
    cycles: u64,
    max_cycles: Option<u64>,

    // JIT related data, should persist across multiple runs
    buffer: Option<Mmap>,
    used: usize,
    freezed: bool,
    // First item in the value tuple is starting address of jitted basic block,
    // second item in the tuple is current cycle count of basic block.
    jitted_blocks: FnvHashMap<usize, (usize, u64)>,
    // This records the start address(first item) and length(second item) of all
    // jitted basic block, so when a memory write happens within a jitted block,
    // we can use this as a hint to clear those jitted blocks since they will be
    // incorrect afther the memory write. Notice this vec is also sorted based on
    // the start address(first item) to allow for binary search operations for speed
    // considerations.
    jitted_segments: Vec<(usize, usize)>,
    // We cannot use generics here since the FFI functions at the end of this
    // file need to stay unmangled, but Rust requires mangled names for
    // functions with generic types.
    tracer: Box<Tracer>,
}

impl RustData {
    fn new(tracer: Box<Tracer>) -> Self {
        Self {
            memory: SparseMemory::default(),
            cycles: 0,
            max_cycles: None,
            buffer: None,
            used: 0,
            freezed: false,
            jitted_blocks: FnvHashMap::default(),
            jitted_segments: vec![],
            tracer,
        }
    }

    fn mark_write(&mut self, offset: usize, length: usize) -> Result<&mut Self, Error> {
        while !self.jitted_segments.is_empty() {
            match self
                .jitted_segments
                .binary_search_by(|(s, e)| match (s + e).cmp(&offset) {
                    Ordering::Greater => match s.cmp(&(offset + length)) {
                        Ordering::Less => Ordering::Equal,
                        _ => Ordering::Greater,
                    },
                    _ => Ordering::Less,
                }) {
                Ok(i) => {
                    // NOTE: there is a quirk that if we have already jitted a
                    // lot of code, several writes could render most (if not
                    // all) blocks unusable. But the other side of the
                    // problem is that allowing relocating JIT is also a lot of
                    // work. Hence we are sticking with the simple solution now.
                    let (s, _) = self.jitted_segments.remove(i);
                    self.jitted_blocks.remove(&s);
                    self.tracer.clear(s)?;
                }
                Err(_) => break,
            }
        }
        Ok(self)
    }
}

#[derive(Default)]
pub struct BaselineJitRunData<'b> {
    max_cycles: Option<u64>,
    instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
    syscalls: Vec<Box<dyn Syscalls<BaselineJitMachine> + 'b>>,
}

impl<'b> BaselineJitRunData<'b> {
    pub fn max_cycles(mut self, max_cycles: u64) -> Self {
        self.max_cycles = Some(max_cycles);
        self
    }

    pub fn instruction_cycle_func(
        mut self,
        instruction_cycle_func: Box<InstructionCycleFunc>,
    ) -> Self {
        self.instruction_cycle_func = Some(instruction_cycle_func);
        self
    }

    pub fn syscall(mut self, syscall: Box<dyn Syscalls<BaselineJitMachine> + 'b>) -> Self {
        self.syscalls.push(syscall);
        self
    }
}

/// A baseline JIT-based machine, the design is to provide a 2-level JIT:
/// * A baseline JIT leveraging similar techniques in qemu's TCG and rv8 to
/// translate RISC-V instructions to native assembly code. Since this level
/// serves as the baseline JIT, JIT compilation speed will take priority over
/// runtime performance of generated code. As a result, only certain but not
/// all optimizations in rv8 would be introduced here, such as macro-op fusion.
/// The baseline JIT here will only work on a basic block boundary.
/// A static register allocation algorithm much like the rv8 one will also be
/// used here. To help with the next level JIT, trace points would also be
/// introduced here for profiling use.
/// * For very hot code, we might introduce a more sophisticated JIT to
/// further optimize those code pieces to further boost the performance. In
/// this level we would leverage algorithms to translate RISC-V instructions
/// to SSA form: http://compilers.cs.uni-saarland.de/papers/bbhlmz13cc.pdf, then
/// apply different optimizations to further optimize the code. We might choose
/// to leverage cranelift or MJIT to enjoy existing work. Note that unlike
/// the above baseline JIT, this path still has many uncertainties which is more
/// likely to change. Also this will has much lower priority if baseline JIT
/// is proved to be enough for CKB use.
pub struct BaselineJitMachine {
    asm_data: AsmData,
    rust_data: Pin<Box<RustData>>,
    // In fact program should not belong here, however we are putting it here
    // so as to shape the API in a way that one instance here only works on
    // one program
    program: Bytes,
}

impl BaselineJitMachine {
    pub fn new(program: Bytes, tracer: Box<Tracer>) -> Self {
        let rust_data = Box::pin(RustData::new(tracer));
        let asm_data = AsmData::new(rust_data.as_ref().get_ref());

        Self {
            asm_data,
            rust_data,
            program,
        }
    }

    pub fn run(self, args: &[Bytes]) -> Result<(i8, Self), Error> {
        self.run_with_data(args, BaselineJitRunData::default())
    }

    pub fn run_with_data<'b>(
        mut self,
        args: &[Bytes],
        data: BaselineJitRunData<'b>,
    ) -> Result<(i8, Self), Error> {
        self.reset();
        self.rust_data.max_cycles = data.max_cycles;
        let program = self.program.clone();
        let mut builder = DefaultMachineBuilder::<Self>::new(self);
        if let Some(instruction_cycle_func) = data.instruction_cycle_func {
            builder = builder.instruction_cycle_func(instruction_cycle_func);
        }
        for syscall in data.syscalls {
            builder = builder.syscall(syscall);
        }
        let mut machine = builder.build();
        machine.load_program(&program, args)?;
        let mut emitter = Emitter::new()?;
        let decoder = build_imac_decoder::<u64>();
        machine.set_running(true);
        while machine.running() {
            let pc = machine.pc().to_usize();
            let jitted_data = {
                let rust_data = &mut machine.inner_mut().rust_data;
                rust_data.tracer.trace(pc)?;
                if let Some((address, cycles)) = rust_data.jitted_blocks.get(&pc) {
                    Some((*address, *cycles))
                } else {
                    None
                }
            };
            if let Some((address, cycles)) = jitted_data {
                machine.add_cycles(cycles)?;
                let core = machine.inner_mut();
                let rust_data = &mut core.rust_data;
                if let Some(buffer) = &rust_data.buffer {
                    let f = unsafe {
                        mem::transmute::<&u8, fn(&mut AsmData)>(&buffer.as_ref()[address])
                    };
                    f(core.asm_data_mut());
                    continue;
                } else {
                    // This should not happen
                    return Err(Error::Unexpected);
                }
            }
            // Fetch next basic block
            let mut current_pc = pc;
            let mut block_length = 0;
            let mut block_cycles = 0;
            let mut instructions = Vec::new();
            loop {
                let instruction = decoder.decode(machine.memory_mut(), current_pc)?;
                let jitable = is_jitable_instruction(instruction);
                let end_instruction = (!jitable) || is_basic_block_end_instruction(instruction);
                // Unjitable instruction will be its own basic block
                if instructions.is_empty() || jitable {
                    let length = instruction_length(instruction);
                    current_pc += length;
                    block_length += length;
                    block_cycles += machine
                        .instruction_cycle_func()
                        .as_ref()
                        .map(|f| f(instruction))
                        .unwrap_or(0);
                    instructions.push(instruction);
                }

                if end_instruction {
                    break;
                }
            }
            for i in &instructions {
                execute(*i, &mut machine)?;
            }
            machine.add_cycles(block_cycles)?;
            let rust_data = &mut machine.inner_mut().rust_data;
            if (!rust_data.freezed)
                && (rust_data
                    .tracer
                    .should_jit(pc, block_length, &instructions)?)
            {
                // current basic block is hot, JIT it.
                let mut compiling_machine = JitCompilingMachine::new(pc);
                for i in &instructions {
                    execute(*i, &mut compiling_machine)?;
                }
                emitter.setup()?;
                for write in compiling_machine.writes() {
                    emitter.emit_write(write)?;
                }
                let encode_size = emitter.link()?;
                let mut buffer_mut = match mem::replace(&mut rust_data.buffer, None) {
                    Some(buffer) => buffer.make_mut()?,
                    None => MmapMut::map_anon(JIT_SEGMENT_LENGTH)?,
                };
                let buffer_mut = if buffer_mut.len() - rust_data.used < encode_size {
                    rust_data.freezed = true;
                    buffer_mut
                } else {
                    // TODO: check if dynasm handles alignments here.
                    emitter.encode(&mut buffer_mut[rust_data.used..])?;
                    let offset = rust_data.used;
                    rust_data.used += encode_size;
                    rust_data.jitted_blocks.insert(pc, (offset, block_cycles));
                    match rust_data
                        .jitted_segments
                        .binary_search_by(|(s, _)| s.cmp(&pc))
                    {
                        // We should not have 2 colliding basic blocks
                        Ok(_) => return Err(Error::Unexpected),
                        Err(i) => rust_data.jitted_segments.insert(i, (pc, block_length)),
                    }
                    buffer_mut
                };
                rust_data.buffer = Some(buffer_mut.make_exec()?);
            }
        }
        Ok((machine.exit_code(), machine.take_inner()))
    }

    fn reset(&mut self) {
        self.asm_data.registers = [0; ASM_DATA_REGISTERS_SLOTS];
        self.rust_data.memory = SparseMemory::<u64>::default();
        self.rust_data.cycles = 0;
        self.rust_data.max_cycles = None;
    }

    fn asm_data_mut(&mut self) -> &mut AsmData {
        &mut self.asm_data
    }
}

impl CoreMachine for BaselineJitMachine {
    type REG = u64;
    type MEM = SparseMemory<u64>;

    fn pc(&self) -> &u64 {
        &self.asm_data.registers[REGISTER_PC]
    }

    fn set_pc(&mut self, next_pc: u64) {
        self.asm_data.registers[REGISTER_PC] = next_pc;
    }

    fn memory(&self) -> &SparseMemory<u64> {
        &self.rust_data.memory
    }

    fn memory_mut(&mut self) -> &mut SparseMemory<u64> {
        &mut self.rust_data.memory
    }

    fn registers(&self) -> &[u64] {
        &self.asm_data.registers[0..REGISTER_PC]
    }

    fn set_register(&mut self, idx: usize, value: u64) {
        self.asm_data.registers[idx] = value;
    }
}

impl SupportMachine for BaselineJitMachine {
    fn cycles(&self) -> u64 {
        self.rust_data.cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.rust_data.cycles = cycles;
    }

    fn max_cycles(&self) -> Option<u64> {
        self.rust_data.max_cycles
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MemorySize {
    Byte = 1,
    HalfWord = 2,
    Word = 4,
    DoubleWord = 8,
}

#[derive(Debug, Clone)]
pub enum Write {
    Memory {
        address: Value,
        size: MemorySize,
        value: Value,
    },
    Register {
        index: usize,
        value: Value,
    },
    Pc {
        value: Value,
    },
}

struct JitCompilingMachine {
    registers: [Value; 33],
    writes: Vec<Write>,
}

impl JitCompilingMachine {
    fn new(pc: usize) -> Self {
        let registers = [
            Value::Register(0),
            Value::Register(1),
            Value::Register(2),
            Value::Register(3),
            Value::Register(4),
            Value::Register(5),
            Value::Register(6),
            Value::Register(7),
            Value::Register(8),
            Value::Register(9),
            Value::Register(10),
            Value::Register(11),
            Value::Register(12),
            Value::Register(13),
            Value::Register(14),
            Value::Register(15),
            Value::Register(16),
            Value::Register(17),
            Value::Register(18),
            Value::Register(19),
            Value::Register(20),
            Value::Register(21),
            Value::Register(22),
            Value::Register(23),
            Value::Register(24),
            Value::Register(25),
            Value::Register(26),
            Value::Register(27),
            Value::Register(28),
            Value::Register(29),
            Value::Register(30),
            Value::Register(31),
            Value::from_usize(pc),
        ];
        Self {
            registers,
            writes: vec![],
        }
    }

    fn writes(&self) -> &[Write] {
        &self.writes
    }
}

// Dummy implementation used to fix build
impl Default for JitCompilingMachine {
    fn default() -> Self {
        Self::new(0)
    }
}

impl CoreMachine for JitCompilingMachine {
    type REG = Value;
    type MEM = Self;

    fn pc(&self) -> &Value {
        &self.registers[REGISTER_PC]
    }

    fn set_pc(&mut self, next_pc: Value) {
        self.writes.retain(|write| match write {
            Write::Pc {
                value: Value::Imm(_),
            } => false,
            _ => true,
        });
        self.writes.push(Write::Pc {
            value: next_pc.clone(),
        });
        self.registers[REGISTER_PC] = next_pc;
    }

    fn memory(&self) -> &Self {
        &self
    }

    fn memory_mut(&mut self) -> &mut Self {
        self
    }

    fn registers(&self) -> &[Value] {
        &self.registers[0..REGISTER_PC]
    }

    fn set_register(&mut self, idx: usize, value: Value) {
        self.writes.retain(|write| match write {
            Write::Register {
                index: idx1,
                value: Value::Imm(_),
            } if *idx1 == idx => false,
            _ => true,
        });
        self.writes.push(Write::Register {
            index: idx,
            value: value.clone(),
        });
        if let Value::Imm(_) = value {
            self.registers[idx] = value;
        } else {
            self.registers[idx] = Value::Register(idx);
        }
    }
}

impl Machine for JitCompilingMachine {
    fn ecall(&mut self) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}

impl Memory<Value> for JitCompilingMachine {
    fn init_pages(
        &mut self,
        _addr: usize,
        _size: usize,
        _flags: u8,
        _source: Option<Bytes>,
        _offset_from_addr: usize,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn fetch_flag(&mut self, _page: usize) -> Result<u8, Error> {
        Err(Error::Unimplemented)
    }

    fn store_byte(&mut self, _addr: usize, _size: usize, _value: u8) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn store_bytes(&mut self, _addr: usize, _value: &[u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn execute_load16(&mut self, _addr: usize) -> Result<u16, Error> {
        Err(Error::Unimplemented)
    }

    fn load8(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), MemorySize::Byte))
    }

    fn load16(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), MemorySize::HalfWord))
    }

    fn load32(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), MemorySize::Word))
    }

    fn load64(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), MemorySize::DoubleWord))
    }

    fn store8(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: MemorySize::Byte,
            value: value.clone(),
        });
        Ok(())
    }

    fn store16(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: MemorySize::HalfWord,
            value: value.clone(),
        });
        Ok(())
    }

    fn store32(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: MemorySize::Word,
            value: value.clone(),
        });
        Ok(())
    }

    fn store64(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: MemorySize::DoubleWord,
            value: value.clone(),
        });
        Ok(())
    }
}

// Following functions are used via FFI in C side.
#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_store8(data: *mut RustData, addr: uint64_t, value: uint64_t) -> c_int {
    unsafe { data.as_mut() }
        .and_then(|data| {
            data.mark_write(addr as usize, 1)
                .and_then(|data| data.memory.store8(&addr, &value))
                .ok()
        })
        .map(|_| 0)
        .unwrap_or(-1)
}

#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_store16(
    data: *mut RustData,
    addr: uint64_t,
    value: uint64_t,
) -> c_int {
    unsafe { data.as_mut() }
        .and_then(|data| {
            data.mark_write(addr as usize, 2)
                .and_then(|data| data.memory.store16(&addr, &value))
                .ok()
        })
        .map(|_| 0)
        .unwrap_or(-1)
}

#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_store32(
    data: *mut RustData,
    addr: uint64_t,
    value: uint64_t,
) -> c_int {
    unsafe { data.as_mut() }
        .and_then(|data| {
            data.mark_write(addr as usize, 4)
                .and_then(|data| data.memory.store32(&addr, &value))
                .ok()
        })
        .map(|_| 0)
        .unwrap_or(-1)
}

#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_store64(
    data: *mut RustData,
    addr: uint64_t,
    value: uint64_t,
) -> c_int {
    unsafe { data.as_mut() }
        .and_then(|data| {
            data.mark_write(addr as usize, 8)
                .and_then(|data| data.memory.store64(&addr, &value))
                .ok()
        })
        .map(|_| 0)
        .unwrap_or(-1)
}

#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_load8(
    data: *mut RustData,
    addr: uint64_t,
    value: *mut uint64_t,
) -> c_int {
    match unsafe { data.as_mut() }.and_then(|data| data.memory.load8(&addr).ok()) {
        Some(v) => {
            if let Some(p) = unsafe { value.as_mut() } {
                *p = v;
            }
            0
        }
        None => -1,
    }
}

#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_load16(
    data: *mut RustData,
    addr: uint64_t,
    value: *mut uint64_t,
) -> c_int {
    match unsafe { data.as_mut() }.and_then(|data| data.memory.load16(&addr).ok()) {
        Some(v) => {
            if let Some(p) = unsafe { value.as_mut() } {
                *p = v;
            }
            0
        }
        None => -1,
    }
}

#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_load32(
    data: *mut RustData,
    addr: uint64_t,
    value: *mut uint64_t,
) -> c_int {
    match unsafe { data.as_mut() }.and_then(|data| data.memory.load32(&addr).ok()) {
        Some(v) => {
            if let Some(p) = unsafe { value.as_mut() } {
                *p = v;
            }
            0
        }
        None => -1,
    }
}

#[no_mangle]
extern "C" fn ckb_vm_jit_ffi_load64(
    data: *mut RustData,
    addr: uint64_t,
    value: *mut uint64_t,
) -> c_int {
    match unsafe { data.as_mut() }.and_then(|data| data.memory.load64(&addr).ok()) {
        Some(v) => {
            if let Some(p) = unsafe { value.as_mut() } {
                *p = v;
            }
            0
        }
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use crate::RISCV_GENERAL_REGISTER_NUMBER;

    #[test]
    fn test_jit_constant_rules() {
        assert!(RISCV_GENERAL_REGISTER_NUMBER == 32);
    }
}
