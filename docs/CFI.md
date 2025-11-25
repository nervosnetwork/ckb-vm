# Design of CFI in CKB-VM

## Preface

This document focuses on the background, purpose, and detailed design of the RISC-V CFI extension. Our previous extension explorations and implementations, including the B extension (implemented) and V extension (implemented but not released), focused on performance improvements. The CFI extension focuses on security protection, which aligns with the positioning and requirements of CKB smart contracts. The latest version of the specification is located at:

https://github.com/riscv/riscv-isa-manual/blob/main/src/unpriv-cfi.adoc

https://github.com/riscv/riscv-isa-manual/blob/main/src/priv-cfi.adoc

## Background

In the RISC-V architecture, the function call mechanism relies on the `ra` (return address) register to store the return address. When executing a function jump, the processor writes the return address into the `ra` register. For nested function call scenarios, since there is only one `ra` register, the current return address in `ra` must be saved to the stack for later restoration.

This design introduces a critical security risk: if an attacker can corrupt the stack contents through some means (such as buffer overflow vulnerabilities), they can tamper with the return address saved on the stack. This is the core principle of [ROP (Return-Oriented Programming)](https://en.wikipedia.org/wiki/Return-oriented_programming) or JOP attacks. Attackers hijack the program's control flow through carefully constructed `gadgets` (code snippets) chains, thereby achieving arbitrary code execution.

In CKB smart contracts, this attack threat is particularly severe. When stack content is corrupted, attackers can possibly bypass all security checks simply by redirecting the return address to `syscall exit` with an exit code of 0, effectively disabling the contract's verification logic. Unlike traditional exploits requiring complex ROP chains, this attack vector is remarkably simple yet devastating in its impact.

Therefore, protection mechanisms for the stack, especially integrity verification of return addresses, are crucial for ensuring the security of CKB-VM. This is also the fundamental motivation for introducing the CFI (Control Flow Integrity) extension.

## Benefiting Scenarios

Currently, smart contracts in the CKB ecosystem are primarily developed using C and Rust. For these two languages, the security enhancement effects of the CFI extension have different focuses:

C language lacks memory safety guarantees and is prone to memory safety vulnerabilities such as buffer overflows, dangling pointers, and use-after-free. These vulnerabilities can all be exploited to corrupt return addresses on the stack, thereby achieving control flow hijacking. The CFI extension can effectively defend against such attacks through hardware-level return address integrity verification, significantly improving the security of C language contracts. This is particularly important for core components such as cryptographic libraries and verification logic that are heavily developed in C in the CKB ecosystem.

Pure Rust code provides memory safety guarantees in safe contexts, and the compiler prevents operations that could lead to stack corruption. Therefore, for contracts written entirely in safe Rust, the protective value of the CFI extension is relatively limited.

However, actual Rust contracts often need to call C libraries through FFI (Foreign Function Interface) or use `unsafe` code blocks in performance-critical paths. These scenarios step outside Rust's safety guarantees and reintroduce memory safety risks. For example, calling C-implemented cryptographic algorithms, directly manipulating raw pointers, and manually managing memory layout can all lead to stack corruption. For these scenarios, the CFI extension can also provide effective protection, serving as an important complement to Rust's memory safety mechanisms.

## CFI Extension Overview

The [RISC-V CFI specification](https://github.com/riscv/riscv-cfi) has been officially merged into the [RISC-V Instruction Set Manual](https://github.com/riscv/riscv-isa-manual). The specification content is divided into the following two parts:
- [Privileged ISA](https://github.com/riscv/riscv-isa-manual/blob/main/src/priv-cfi.adoc): Privileged instruction set architecture, defining CFI support at the operating system and hypervisor level
- [Unprivileged ISA](https://github.com/riscv/riscv-isa-manual/blob/main/src/unpriv-cfi.adoc): Unprivileged instruction set architecture, defining CFI instructions at the application level

From the maturity of the specification, the CFI extension specification has entered a stable phase. For CKB-VM, the core focus is on the Unprivileged ISA part, which introduces the following 5 new instructions:
- `LPAD` (Landing Pad): Marks legal indirect jump target locations for forward-edge protection
- `SSPUSH` (Shadow Stack Push): Pushes the return address onto the shadow stack
- `SSPOPCHK` (Shadow Stack Pop and Check): Pops the return address from the shadow stack and verifies its integrity
- `SSRDP` (Shadow Stack Read Pointer): Reads the shadow stack pointer
- `SSAMOSWAP` (Shadow Stack Atomic Swap): Atomically swaps values on the shadow stack

The core mechanism of these instructions is the Shadow Stack: in addition to the regular program stack, the hardware maintains an independent shadow stack dedicated to storing return addresses. When a function call occurs, the return address is saved on both the regular stack and the shadow stack; upon function return, the hardware verifies that the return addresses on both stacks are consistent. Since the shadow stack is invisible to normal memory access instructions, even if attackers can corrupt the regular stack, they cannot synchronously tamper with the shadow stack, thus achieving return address integrity protection.

## LLVM Toolchain Status

Regarding LLVM support for RISC-V CFI extension instructions, it is currently in the development phase. Considering that the CFI specification has been officially incorporated into the RISC-V instruction set manual, toolchain support is expected to be completed in the near future. You can continue to track: https://github.com/search?q=repo%3Allvm%2Fllvm-project+Zicfiss&type=commits

In llvm 21, experimental switches can be enabled through the following command line:

```bash
--target=riscv64 
-march=rv64imc_zba_zbb_zbc_zbs_zicfiss1p0_zicfilp1p0 
-menable-experimental-extensions 
-fcf-protection=full 
-mcf-branch-label-scheme=func-sig
```

For the `zicfiss` part, the implementation is relatively complete and can generate related code; for the `zicfilp` part, the parameter for generating lpad instructions is always 0. When -mcf-branch-label-scheme=func-sig is used, the lpad parameter should be a unique number determined by the function and its parameters. However, this part has not yet been completed.

## Implementation Challenges

Implementing the CFI extension in CKB-VM has the following technical challenges that require special attention during design.

### CSR Instruction Introduction

It is currently unclear whether CSR (Control and Status Registers) related instructions need to be supported. The CFI specification mentions a typical scenario that requires CSR instructions to manipulate the shadow stack pointer:

```asm
csrrw       ra, ssp, ra        # swap ssp: ra=ssp, ssp=ra
```

CSR instructions are needed to support setjmp/longjmp:

```C
longjmp() {
    :
    // Read current shadow stack pointer and
    // compute number of call frames to unwind
    asm("ssrdp %0" : "=r"(cur_ssp):);
    // Skip the unwind if backward-edge CFI not active
    asm("beqz %0, back_cfi_not_active" : "=r"(cur_ssp):);
    // Unwind the frames in a loop
    while ( jmp_buf->saved_ssp > cur_ssp ) {
        // advance by a maximum of 4K at a time to avoid
        // unwinding past bounds of the shadow stack
        cur_ssp = ( (jmp_buf->saved_ssp - cur_ssp) >= 4096 ) ?
                (cur_ssp + 4096) : jmp_buf->saved_ssp;
        asm("csrw ssp, %0" : :  "r" (cur_ssp));
        // Test if unwound past the shadow stack bounds
        asm("sspush x5");
        asm("sspopchk x5");
    }
back_cfi_not_active:
    :
}
```

If mainstream compiler toolchains generate such instructions, CKB-VM will not only need to implement the 5 core CFI instructions but also need to additionally introduce CSR-related instruction support. This will increase implementation complexity and the scope of instruction set extension, requiring detailed analysis of the compiler's code generation patterns. The spec states:

> The Zicfilp extension depends on the Zicsr extension.
> 

> The Zicfiss extension depends on the Zicsr and Zimop extensions.
> 

Here we need to observe how the compiler uses these `Zicsr` extension instructions. If the compiler does not actively generate CSR instructions, we have other options, such as implementing syscall instructions to support operations on `ssp`. The current implementation strategy is that `Zicsr` and `Zimop` instructions will not be implemented separately.

### Zimop/Zcmop Extension

The Zicfiss extension instructions depend on the Zimop/Zcmop extension instructions.

> The Zicfiss instructions, except `SSAMOSWAP.W/D`, are encoded using a subset of May-Be-Operation instructions defined by the Zimop and Zcmop extensions. This subset of instructions revert to their Zimop/Zcmop defined behavior when the Zicfiss extension is not implemented or if the extension has not been activated.
> 

Zimop/Zcmop instructions are essentially close to no-op instructions, similar to placeholders. When `Zicfiss` is not enabled, except for the `SSAMOSWAP` instruction, `Zicfiss` instructions fallback to Zimop/Zcmop instructions.

Therefore, the recommended shadow stack implementation logic is as follows:

1. When ISA_CFI is not set, it means CFI is completely unsupported, and similar instructions should report errors at the decode stage.
2. When ISA_CFI is set, but the activation flag in the ELF is not activated, these instructions are translated to no-ops at the decode stage (ssrdp has special handling)
3. When ISA_CFI is set and the activation flag in the ELF is activated, these instructions behave as normal functional CFI instructions.

The recommended logic for the LPAD instruction is as follows:

1. When ISA_CFI is not set, it means CFI is completely unsupported. However, LPAD itself is a HINT instruction, which the current CKB-VM can handle, equivalent to a no-op.
2. When ISA_CFI is set, but the activation flag in the ELF is not activated, the instruction is translated to a no-op at the decode stage
3. When ISA_CFI is set and the activation flag in the ELF is activated, the instruction is still translated to a no-op, but checking is enabled

It is worth noting that `Zimop` instructions require:

- MOPs are initially defined to simply write zero to `x[rd]`, but are designed to be redefined by later extensions to perform some other action.

This is particularly important for `ssrdp`, which will set the rd register to 0. sspush and sspopchk will not perform similar operations, equivalent to no-ops.

### Memory Protection Mechanism

Some mechanisms involved in the Unprivileged ISA specification, such as [shadow stack memory protection](https://github.com/riscv/riscv-isa-manual/blob/main/src/priv-cfi.adoc#shadow-stack-memory-protection), are described relatively abstractly in the current specification. These mechanisms need to be designed in depth in combination with CKB-VM's memory model during actual implementation to ensure isolation between the shadow stack and the regular stack while avoiding introducing new security vulnerabilities or performance bottlenecks.

We can design an independent and isolated memory region dedicated to serving as the shadow stack. In this approach, since it does not occupy the 4M memory, the shadow stack size can be appropriately increased to 64KB. The shadow stack is stored as type `[u8; 64 * 1024]`. The 64KB memory can store 8K return addresses, which is sufficient. Nested calls with levels exceeding 8K are either extremely rare or would result in a stack overflow scenario.

The advantages of this approach are:

- No need to introduce new memory page flags
- No need to modify the implementation of previous store/load instructions
- No need to modify the previous memory layout

The disadvantage is that snapshots need to store more data.

### Stack Unwinding (setjmp/longjmp, C++ Exception Handling) Implementation Patterns

In programs with shadow stack enabled, the regular stack and shadow stack must remain synchronized. When non-sequential control flow transfers occur (such as longjmp, C++ exception handling), the regular stack is directly restored to a previously saved state, but the shadow stack must also be synchronously restored to the corresponding position. If the two stacks are not synchronized, subsequent function returns will trigger exceptions due to return address verification failures. 

### Activation Flag in ELF

In addition to the CKB-VM flag itself, the spec requires an additional ELF flag. Consider a scenario: a contract that has not enabled the `Zicfilp` extension (such as an old contract) needs to be guaranteed to still run normally. Because it does not contain `LPAD` instructions, and according to the specification, when this extension is enabled, the system will perform checks, and if the corresponding LPAD instruction is missing, it will directly report an error. Therefore, there is an additional flag in the ELF file. The spec states:

> Compilers and linkers should provide an attribute flag to indicate if the program has been compiled with the Zicfilp extension and use that to determine if the Zicfilp extension should be activated.
> 

When this additional flag is not enabled, LPAD will be treated as a no-op. This flag depends on the compiler's implementation. LLVM currently places the information in the `.note.gnu.property` section.

It is worth noting that `Zicfiss` also has the same flag. You can use llvm-readelf to read the flag:

```bash
llvm-readelf -n build/fib
Displaying notes found in: .note.gnu.property
Owner                Data size 	Description
GNU                  0x00000010	NT_GNU_PROPERTY_TYPE_0 (property note)
    Properties:    RISC-V feature: ZICFISS
```

See [code](https://github.com/llvm/llvm-project/blob/66b481556e01e6e2508d7c9146849167b9e0323f/llvm/include/llvm/BinaryFormat/ELF.h#L1909-L1913):

```C
// RISC-V processor feature bits.
enum : unsigned {
GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_UNLABELED = 1 << 0,
GNU_PROPERTY_RISCV_FEATURE_1_CFI_SS = 1 << 1,
GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_FUNC_SIG = 1 << 2,
};
```

These flags are very useful. For example, in the implementation, when GNU_PROPERTY_RISCV_FEATURE_1_CFI_SS is not enabled, all shadow stack related instructions can be directly translated to no-ops at the decode stage.

### Linking Mixed Objects

When linking modules with different CFI compilation parameters, if one object does not have CFI enabled, then the final generated ELF will not enable CFI. This is manifested by the absence of the corresponding activation flag when using the `llvm-readelf` command.

This issue can be quite troublesome in practice. As long as one third-party library does not enable CFI, the entire contract's CFI protection will be disabled. We may need to provide a  mechanism to help developers quickly locate the problem when CFI is disabled. It can also be achieved by passing the parameter `-z zicfiss-report=warning` to ld.lld, which will report the following warning:

```bash
# REPORT-WARN: warning: f2.o: -z zicfiss-report: file does not have GNU_PROPERTY_RISCV_FEATURE_1_CFI_SS property
```

It is worth noting that to fully support CFI, both the compiler and C runtime must support it. This means that if the support progress of these two lags behind, it will cause the toolchain's progress to lag. Currently, we have found that the LPAD activation flag in the ELF is not enabled, which may be related to this factor.

## Acceptance Criteria

The implementation acceptance of the CFI extension requires comprehensive evaluation from multiple dimensions to ensure functional correctness, acceptable performance, and ecosystem compatibility. The following are the main acceptance criteria:

### Performance Benchmarking

The shadow stack operations introduced by the CFI extension will bring additional performance overhead to program execution. Representative C contracts need to be selected as benchmarks (such as cryptographic verification contracts, complex business logic contracts, etc.), and their execution cycle counts should be measured separately with CFI enabled and disabled to quantify the specific value of performance loss. Additionally, it is necessary to analyze the source distribution of performance loss and identify potential optimization opportunities.

### Specification Compliance Verification

The RISC-V CFI specification should include an official test suite to verify whether the implementation strictly follows the semantic definitions of the specification. It is necessary to investigate and obtain these official test cases to ensure that CKB-VM's CFI implementation can pass all tests.

### Toolchain Integration Testing

In real development scenarios, CFI instructions are automatically generated by compiler toolchains rather than manually written. Therefore, end-to-end integration testing with toolchains such as LLVM and GCC is required at a minimum. Specifically, CFI compilation options should be enabled for typical C contracts existing in the CKB ecosystem, the instruction sequences generated by the compiler should be observed, and it should be verified that CKB-VM can correctly execute these CFI codes generated by the toolchain. Through toolchain integration testing, blind spots and compatibility issues in the implementation can be discovered early.

We hope to get involved and start testing during the toolchain development phase to discover incompatibility issues early and provide feedback to upstream developers. For example, if LLVM generates CFI instructions while introducing instructions that third parties do not support, we will face a difficult choice—introducing 5 CFI instructions would require supporting additional instructions, which is not cost-effective.

## Conclusion

The RISC-V CFI extension provides security protection for CKB-VM through the shadow stack mechanism, effectively defending against ROP/JOP attacks. This extension only involves few new instructions, with controllable implementation complexity, yet can significantly improve the security of C language contracts and Rust contracts containing unsafe code.

The CFI specification has been officially incorporated into the RISC-V instruction set manual, ensuring standard stability. Although support from toolchains such as LLVM is still being improved, the trend is clear. It is recommended that CKB-VM complete instruction-level implementation and testing in advance, so that it can be quickly deployed once the toolchain matures. This is a cost-effective security upgrade solution.
