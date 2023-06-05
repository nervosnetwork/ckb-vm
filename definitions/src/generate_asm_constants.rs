use ckb_vm_definitions::{
    asm::{
        AsmCoreMachine, Trace, RET_CYCLES_OVERFLOW, RET_DECODE_TRACE, RET_DYNAMIC_JUMP, RET_EBREAK,
        RET_ECALL, RET_INVALID_PERMISSION, RET_MAX_CYCLES_EXCEEDED, RET_OUT_OF_BOUND, RET_SLOWPATH,
        RET_SUSPEND, TRACE_ITEM_LENGTH,
    },
    instructions::{
        instruction_opcode_name, Instruction, INSTRUCTION_OPCODE_NAMES, MAXIMUM_OPCODE,
        MINIMAL_OPCODE,
    },
    memory::{FLAG_DIRTY, FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT},
    registers::{RA, SP},
    MEMORY_FRAMES, MEMORY_FRAMESIZE, MEMORY_FRAME_PAGE_SHIFTS, MEMORY_FRAME_SHIFTS,
    RISCV_MAX_MEMORY, RISCV_PAGES, RISCV_PAGESIZE, RISCV_PAGE_SHIFTS,
};
use std::mem::{size_of, zeroed};

// This utility helps us generate C-based macros containing definitions
// such as return code, opcode, struct size, struct offset, etc. The exact
// data here are derived while inspecting Rust structs dynamically. We keep
// this in a separate crate so build failures from the main crate won't cause
// a problem when updating the definitions with this crate. Or you can think
// of this as a workaround to the problem that build.rs cannot depend on any
// of its crate contents.
fn main() {
    println!("#define CKB_VM_ASM_RISCV_MAX_MEMORY {}", RISCV_MAX_MEMORY);
    println!("#define CKB_VM_ASM_RISCV_PAGE_SHIFTS {}", RISCV_PAGE_SHIFTS);
    println!("#define CKB_VM_ASM_RISCV_PAGE_SIZE {}", RISCV_PAGESIZE);
    println!("#define CKB_VM_ASM_RISCV_PAGE_MASK {}", RISCV_PAGESIZE - 1);
    println!("#define CKB_VM_ASM_RISCV_PAGES {}", RISCV_PAGES);
    println!(
        "#define CKB_VM_ASM_MEMORY_FRAME_SHIFTS {}",
        MEMORY_FRAME_SHIFTS
    );
    println!("#define CKB_VM_ASM_MEMORY_FRAMESIZE {}", MEMORY_FRAMESIZE);
    println!("#define CKB_VM_ASM_MEMORY_FRAMES {}", MEMORY_FRAMES);
    println!(
        "#define CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS {}",
        MEMORY_FRAME_PAGE_SHIFTS
    );
    println!();

    println!(
        "#define CKB_VM_ASM_MAXIMUM_TRACE_ADDRESS_LENGTH {}",
        TRACE_ITEM_LENGTH * 4
    );
    println!();

    println!("#define CKB_VM_ASM_RET_DECODE_TRACE {}", RET_DECODE_TRACE);
    println!("#define CKB_VM_ASM_RET_ECALL {}", RET_ECALL);
    println!("#define CKB_VM_ASM_RET_EBREAK {}", RET_EBREAK);
    println!("#define CKB_VM_ASM_RET_DYNAMIC_JUMP {}", RET_DYNAMIC_JUMP);
    println!(
        "#define CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED {}",
        RET_MAX_CYCLES_EXCEEDED
    );
    println!(
        "#define CKB_VM_ASM_RET_CYCLES_OVERFLOW {}",
        RET_CYCLES_OVERFLOW
    );
    println!("#define CKB_VM_ASM_RET_OUT_OF_BOUND {}", RET_OUT_OF_BOUND);
    println!(
        "#define CKB_VM_ASM_RET_INVALID_PERMISSION {}",
        RET_INVALID_PERMISSION
    );
    println!("#define CKB_VM_ASM_RET_SLOWPATH {}", RET_SLOWPATH);
    println!("#define CKB_VM_ASM_RET_SUSPEND {}", RET_SUSPEND);
    println!();

    println!("#define CKB_VM_ASM_REGISTER_RA {}", RA);
    println!("#define CKB_VM_ASM_REGISTER_SP {}", SP);
    println!();

    println!("#define CKB_VM_ASM_MEMORY_FLAG_FREEZED {}", FLAG_FREEZED);
    println!(
        "#define CKB_VM_ASM_MEMORY_FLAG_EXECUTABLE {}",
        FLAG_EXECUTABLE
    );
    println!(
        "#define CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT {}",
        FLAG_WXORX_BIT
    );
    println!("#define CKB_VM_ASM_MEMORY_FLAG_WRITABLE {}", FLAG_WRITABLE);
    println!("#define CKB_VM_ASM_MEMORY_FLAG_DIRTY {}", FLAG_DIRTY);
    println!();

    println!(
        "#define CKB_VM_ASM_TRACE_STRUCT_SIZE {}",
        size_of::<Trace>()
    );

    let t: Trace = unsafe { zeroed() };
    let t_address = &t as *const Trace as usize;
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_ADDRESS {}",
        (&t.address as *const u64 as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_LENGTH {}",
        (&t.length as *const u8 as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_CYCLES {}",
        (&t.cycles as *const u64 as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_INSTRUCTIONS {}",
        (&t.instructions as *const Instruction as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_THREAD {}",
        (&t.thread as *const u64 as usize) - t_address
    );
    println!();

    let m: Box<AsmCoreMachine> = AsmCoreMachine::new(0, 0, 0);
    let m_address = &*m as *const AsmCoreMachine as usize;
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS {}",
        (&m.registers as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC {}",
        (&m.pc as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES {}",
        (&m.cycles as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES {}",
        (&m.max_cycles as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CHAOS_MODE {}",
        (&m.chaos_mode as *const u8 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CHAOS_SEED {}",
        (&m.chaos_seed as *const u32 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LOAD_RESERVATION_ADDRESS {}",
        (&m.load_reservation_address as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_SIZE {}",
        (&m.memory_size as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES_SIZE {}",
        (&m.frames_size as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS_SIZE {}",
        (&m.flags_size as *const u64 as usize) - m_address
    );

    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_READ_FRAME {}",
        (&m.last_read_frame as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_WRITE_PAGE {}",
        (&m.last_write_page as *const u64 as usize) - m_address
    );

    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS {}",
        (&m.flags as *const u8 as usize) - m_address
    );
    let memory_offset_address = (&m.memory as *const u8 as usize) - m_address;
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY {}",
        memory_offset_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES {}",
        (&m.traces as *const Trace as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES {}",
        (&m.frames as *const u8 as usize) - m_address
    );
    println!();

    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_H {}",
        memory_offset_address.wrapping_shr(12).wrapping_shl(12)
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_L {}",
        memory_offset_address & 0xFFF
    );

    println!();

    for op in MINIMAL_OPCODE..MAXIMUM_OPCODE {
        println!(
            "#define CKB_VM_ASM_OP_{} {}",
            instruction_opcode_name(op),
            op
        );
    }
    println!();

    println!("#ifdef CKB_VM_ASM_GENERATE_LABEL_TABLES");
    println!("#ifdef __APPLE__");
    println!(".global _ckb_vm_asm_labels");
    println!("_ckb_vm_asm_labels:");
    println!("#else");
    println!(".global ckb_vm_asm_labels");
    println!("ckb_vm_asm_labels:");
    println!("#endif");
    println!(".CKB_VM_ASM_LABEL_TABLE:");
    for _ in 0..0x10 {
        println!("\t.long\t.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE");
    }
    for name in INSTRUCTION_OPCODE_NAMES.iter() {
        println!(
            "\t.long\t.CKB_VM_ASM_LABEL_OP_{} - .CKB_VM_ASM_LABEL_TABLE",
            name
        );
    }
    println!("#endif /* CKB_VM_ASM_GENERATE_LABEL_TABLES */");
}
