# ASM Code Execution Flow in CKB-VM

This document explains the step-by-step process of executing RISC-V code using the Assembly (ASM) optimized execution mode in CKB-VM.

## Overview

The ASM execution mode is a high-performance execution engine that uses:
- **Hand-written x64 assembly** for instruction execution (`src/machine/asm/execute.S`)
- **Trace-based execution** with instruction caching
- **Direct threaded code** for fast instruction dispatch
- **Optional AOT (Ahead-of-Time) compilation** for even better performance

This mode is only available on 64-bit Linux, macOS, and Windows platforms when the `asm` or `detect-asm` feature is enabled.

## Key Components

### 1. AsmCoreMachine (`definitions/src/asm.rs:36`)
The core machine structure used in ASM execution:
```c
struct AsmCoreMachine {
    registers: [u64; 32],        // RISC-V registers
    pc: u64,                     // Program counter
    running: u8,                 // Execution state
    cycles: u64,                 // Current cycle count
    max_cycles: u64,             // Maximum allowed cycles
    flags: [u8; RISCV_PAGES],    // Page permission flags
    memory: [u8; RISCV_MAX_MEMORY], // 4MB memory
    traces: [Trace; 8192],       // Cached instruction traces
}
```

### 2. Trace Structure (`definitions/src/asm.rs:25`)
Each trace caches a basic block of up to 16 instructions:
```rust
struct Trace {
    address: u64,                       // Starting PC address
    length: u8,                         // Total bytes of instructions
    cycles: u64,                        // Cycle cost of this trace
    instructions: [Instruction; 17],    // Decoded instructions
    thread: [u64; 17],                  // Direct threaded code addresses
}
```

### 3. Assembly Execute Function
```c
extern "C" uint8_t ckb_vm_x64_execute(AsmCoreMachine* m);
```
Hand-written assembly that executes instructions directly.

## Execution Steps

### Step 1: Initialization

```rust
// Create ASM machine
let mut asm_machine = AsmMachine::new(
    DefaultMachine::<Box<AsmCoreMachine>>::default(),
    Some(&aot_code)  // Optional AOT code
);

// Load ELF program
asm_machine.load_program(&program, &args)?;
```

**What happens:**
1. `AsmCoreMachine` is allocated with `alloc_zeroed()` (takes ~1ms for 4MB memory)
2. `max_cycles` is set (defaults to `u64::MAX` if not specified)
3. ELF program is parsed and loaded into memory
4. Memory pages are marked with permission flags (executable/writable)
5. Stack is initialized with program arguments
6. PC is set to the entry point from ELF header

### Step 2: Main Execution Loop

The `run()` method (`src/machine/asm/mod.rs:254`) enters the main loop:

```rust
self.machine.set_running(true);
while self.machine.running() {
    // Execute with AOT or ASM
    let result = if let Some(aot_code) = &self.aot_code {
        // Check if AOT code exists for this PC
        if let Some(offset) = aot_code.labels.get(self.machine.pc()) {
            // Execute AOT compiled code
            let f = transmute::<u64, fn(*mut AsmCoreMachine, u64) -> u8>(base_address);
            f(&mut machine, offset_address)
        } else {
            // Fall back to ASM interpreter
            ckb_vm_x64_execute(&mut machine)
        }
    } else {
        // Pure ASM execution
        ckb_vm_x64_execute(&mut machine)
    }

    // Handle result
    match result { ... }
}
```

### Step 3: Trace Execution (Inside `ckb_vm_x64_execute`)

The assembly code follows this flow:

#### 3.1 Trace Lookup

```
1. Get current PC from machine.pc
2. Calculate trace slot: slot = ((pc >> 9) + pc) >> 1 & 0x1FFF
3. Get trace from machine.traces[slot]
4. Check if trace is valid:
   - Does trace.address == current PC?
   - Is trace populated (has instructions)?
```

**Trace Hit:** If valid, jump to step 3.3 (execute cached trace)
**Trace Miss:** If invalid, return `RET_DECODE_TRACE` to Rust

#### 3.2 Trace Decoding (Rust Side)

When assembly returns `RET_DECODE_TRACE` (`src/machine/asm/mod.rs:273`):

```rust
let pc = *self.machine.pc();
let slot = calculate_slot(pc);
let mut trace = Trace::default();
let mut current_pc = pc;
let mut i = 0;

// Decode up to 16 instructions
while i < TRACE_ITEM_LENGTH {
    let instruction = decoder.decode(self.machine.memory_mut(), current_pc)?;
    let end_instruction = is_basic_block_end_instruction(instruction);
    current_pc += instruction_length(instruction);

    // Store instruction with offset in unused bits
    instruction |= ((current_pc - pc) as u8) << 24;
    trace.instructions[i] = instruction;

    // Calculate cycle cost
    trace.cycles += self.machine.instruction_cycle_func()
        .as_ref().map(|f| f(instruction)).unwrap_or(0);

    // Get direct threaded code address
    let opcode = extract_opcode(instruction);
    trace.thread[i] = asm_labels[opcode] + labels_base_address;

    i += 1;
    if end_instruction { break; }  // Stop at branches/jumps
}

// Add end marker
trace.instructions[i] = OP_CUSTOM_TRACE_END;
trace.thread[i] = asm_labels[OP_CUSTOM_TRACE_END] + labels_base_address;
trace.address = pc;
trace.length = (current_pc - pc) as u8;

// Store trace
self.machine.traces[slot] = trace;
```

**Basic block end conditions:**
- Branch instructions (BEQ, BNE, BLT, BGE, BLTU, BGEU, BEQZ, BNEZ)
- Jump instructions (JAL, JALR, J, JR)
- System instructions (ECALL, EBREAK)
- AUIPC (can change PC calculation)

#### 3.3 Trace Execution (Assembly Side)

Once trace is decoded, the assembly code uses **direct threaded code**:

```asm
; Get first thread address
mov r10, [trace.thread]

; Jump to instruction handler
jmp [r10]

; Each instruction handler:
instruction_add:
    ; Execute ADD operation
    mov rax, [machine.registers + rs1*8]
    add rax, [machine.registers + rs2*8]
    mov [machine.registers + rd*8], rax

    ; Get next thread address
    add inst_pc, 8  ; Move to next instruction
    mov r10, [inst_pc]

    ; Direct jump to next instruction
    jmp [r10]

instruction_trace_end:
    ; Update PC
    movzx rax, byte [inst_pc + offset_field]
    add [machine.pc], rax

    ; Update cycles
    mov rax, [trace.cycles]
    add [machine.cycles], rax

    ; Check cycle limit
    mov rdx, [machine.max_cycles]
    cmp [machine.cycles], rdx
    ja cycles_exceeded

    ; Return to Rust to get next trace
    mov al, RET_DYNAMIC_JUMP
    ret
```

**Direct threading benefits:**
- No instruction decode in the hot path
- No switch/case dispatch overhead
- CPU branch predictor works efficiently

### Step 4: Special Operations

During trace execution, the assembly may return control to Rust for:

#### RET_ECALL (System Call)
```rust
self.machine.ecall()?;
```
- Reads syscall number from register A7 (x17)
- If code is 93: exit program with code from A0
- Otherwise: dispatch to registered syscall handlers

#### RET_EBREAK (Breakpoint)
```rust
self.machine.ebreak()?;
```
- Calls the debugger if one is registered
- Otherwise: no-op (continues execution)

#### RET_DYNAMIC_JUMP
```rust
() // Continue to next iteration
```
- Normal case: trace executed successfully
- Loop back to step 3 to find/execute next trace

#### RET_MAX_CYCLES_EXCEEDED
```rust
return Err(Error::InvalidCycles);
```
- Cycle limit reached, stop execution

#### RET_OUT_OF_BOUND
```rust
return Err(Error::OutOfBound);
```
- Memory access outside valid range

#### RET_INVALID_PERMISSION
```rust
return Err(Error::InvalidPermission);
```
- Attempted to write to executable memory (W^X violation)
- Attempted to execute non-executable memory

### Step 5: Exit

Execution completes when:
- Program calls `exit()` syscall (code 93)
- Error occurs (out of bounds, invalid permission, etc.)
- Cycle limit exceeded

```rust
self.machine.set_running(false);
Ok(self.machine.exit_code())
```

## AOT (Ahead-of-Time) Execution

When AOT code is provided (`src/machine/asm/mod.rs:258`):

### AOT Compilation Process (Optional Pre-step)

1. **Label Gathering**: Parse ELF and identify all code entry points
2. **Block Building**: For each label, build instruction blocks
3. **Code Emission**: Generate native x64 code using DynASM
4. **Memory Mapping**: Store in executable memory pages

### AOT Execution

```rust
if let Some(offset) = aot_code.labels.get(self.machine.pc()) {
    let base_address = aot_code.base_address();
    let offset_address = base_address + offset;

    // Call directly into AOT compiled native code
    let f = transmute::<u64, fn(*mut AsmCoreMachine, u64) -> u8>(base_address);
    f(&mut machine, offset_address)
}
```

**Benefits:**
- No trace decoding overhead
- Optimized native code generated once
- Direct native execution of RISC-V instructions
- Can optimize across instructions

**Fallback:**
- If PC is not in AOT labels, falls back to ASM interpreter
- Seamless transition between AOT and ASM modes

## Performance Characteristics

### Trace Execution (ASM Mode)
- **First execution**: Decode trace (slower)
- **Subsequent executions**: Direct threaded code (very fast)
- **Trace size**: Up to 16 instructions per trace
- **Trace cache**: 8192 slots (LRU-style via hash)

### Memory Layout
- **Total size**: ~4MB (RISCV_MAX_MEMORY)
- **Page size**: 4KB (RISCV_PAGESIZE)
- **Total pages**: 1024
- **Allocation**: Single `alloc_zeroed()` call (~1ms overhead)

### Cycle Counting
- Calculated during trace decode
- Updated once per trace (not per instruction)
- Checked at trace end
- Optional: can be disabled by not providing `instruction_cycle_func`

## Security Features

### W^X (Write XOR Execute) Enforcement
```asm
CHECK_WRITE_PERMISSION:
    ; Get page number
    shr address, 12

    ; Check flag
    movzx flag, byte [machine.flags + page]
    test flag, FLAG_WRITABLE
    jz permission_denied

    test flag, FLAG_EXECUTABLE
    jnz permission_denied  ; Cannot write to executable pages
```

### Bounds Checking
Every memory access checks:
```asm
cmp address, RISCV_MAX_MEMORY
jae out_of_bound
```

## Comparison: ASM vs TraceMachine vs Interpreter

| Feature | Interpreter | TraceMachine (Pure Rust) | ASM Mode | ASM + AOT |
|---------|-------------|--------------------------|----------|-----------|
| Decode per instruction | Yes | Once per trace | Once per trace | Once at compile |
| Dispatch method | Function call | Function call | Direct threading | Direct jump |
| Cycle counting | Per instruction | Per instruction | Per trace | Per trace |
| Platform | All | All | x64 only | x64 only |
| Performance | 1x | ~2-3x | ~10-20x | ~20-50x |

## Code References

- **ASM Machine**: `src/machine/asm/mod.rs:235`
- **Execute function**: `src/machine/asm/execute.S`
- **Trace structure**: `definitions/src/asm.rs:25`
- **AsmCoreMachine**: `definitions/src/asm.rs:36`
- **AOT compilation**: `src/machine/aot/mod.rs`
- **Main run loop**: `src/machine/asm/mod.rs:254`
- **Trace decoding**: `src/machine/asm/mod.rs:273`

## Usage Example

```rust
use ckb_vm::{
    machine::{asm::AsmMachine, DefaultCoreMachine},
    DefaultMachine, DefaultMachineBuilder, SparseMemory
};
use bytes::Bytes;

// Load program
let program = Bytes::from(std::fs::read("program.elf")?);

// Create ASM machine
let core_machine = Box::<AsmCoreMachine>::default();
let builder = DefaultMachineBuilder::new(core_machine)
    .instruction_cycle_func(Box::new(|_| 1)); // 1 cycle per instruction

let mut machine = AsmMachine::new(builder.build(), None);

// Load and run
machine.load_program(&program, &[])?;
let exit_code = machine.run()?;
```

## Debugging

To debug ASM execution:
1. Use `TraceMachine` first to verify correctness (pure Rust)
2. Compare outputs between TraceMachine and AsmMachine
3. Check trace generation by printing trace contents
4. Use GDB with assembly stepping (requires debug symbols)
5. Enable verbose logging in `execute.S` (custom modifications)

## Limitations

- **Platform**: 64-bit x64 only (Linux, macOS, Windows)
- **Memory**: Fixed 4MB allocation (takes ~1ms)
- **Trace cache**: 8192 slots (potential conflicts)
- **Trace length**: Maximum 16 instructions per trace
- **Assembly maintenance**: Hand-written assembly is harder to maintain
- **Build complexity**: Requires C compiler and assembler

## Future Optimizations

Potential improvements mentioned in comments:
- Lazy memory allocation instead of `alloc_zeroed()`
- Larger trace sizes for longer basic blocks
- Better trace cache eviction strategies
- SIMD instructions for bulk operations
- Profile-guided optimization for AOT
