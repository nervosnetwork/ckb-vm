# CKB VM Project AI Collaboration Guidelines

## Project Overview

- This repository is the virtual machine implementation for Nervos CKB.
- CKB VM is a pure software RISC-V virtual machine for running CKB Script, supporting both 32-bit and 64-bit register modes. In real production environments, only the 64-bit mode is used. Supported arch format is RV64IMC_ZBA_ZBB_ZBC_ZBS.
- The project includes both a Rust interpreter and an assembly-based ASM interpreter. ASM mode should be preferred in production; Rust mode is mainly for development assistance. When behavior differs between the two, it must be clearly documented.

## Development and Build Commands

```bash
# Quick compile check
cargo check --all --all-targets --all-features

# Default tests
make test

# ASM interpreter tests
make test-asm

# Formatting check
make fmt

# Clippy check
make clippy

# Full CI-level checks
make ci
```

## Working Principles

1. The project supports multiple memory models, but production environments use only `WXorXMemory<SparseMemory>`. Other memory models are intended only to assist development. When memory models behave differently, production behavior takes precedence. The W^X memory model must keep code pages non-writable, data pages non-executable, and page flags correctly maintained.
2. CKB-VM uses internal version numbers to distinguish hard-fork versions. Code selects behavior for each hard-fork version by comparing these version numbers. Known bugs may be fixed in newer versions, while preserving them in older versions is expected behavior for an on-chain VM.
3. Cycles are part of the consensus mechanism, and any change to cycle accounting may cause a consensus fork. Cycle consumption should also remain broadly proportional to actual CPU time to prevent denial-of-service attacks.
4. There are two Snapshot implementations, but only Snapshot2 is used in production. Snapshot2 must correctly save and restore the complete executable state.
5. ASM constants and generated files must be derived from the source definitions in `definitions`. After changing constants, flags, return codes, or the ASM ABI, run `make update-cdefinitions` and ensure that the generated files have no uncommitted differences.
6. When modifying code, you should try to think for yourself as much as possible, rather than searching online code repositories to copy existing code.

## Boundaries and Prohibited Actions

- When a security vulnerability is discovered, it must not be disclosed publicly in any form, including in issues, PRs, discussion forums, or chat groups.
