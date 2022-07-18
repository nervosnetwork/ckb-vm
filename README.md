# [Nervos CKB](https://nervos.org) VM

[![Build Status](https://travis-ci.com/nervosnetwork/ckb-vm.svg?branch=master)](https://travis-ci.com/nervosnetwork/ckb-vm)
[![Build Status](https://dev.azure.com/nervosnetwork/ckb-vm/_apis/build/status/nervosnetwork.ckb-vm?branchName=develop)](https://dev.azure.com/nervosnetwork/ckb-vm/_build/latest?definitionId=10&branchName=develop)
[![codecov](https://codecov.io/gh/nervosnetwork/ckb-vm/branch/develop/graph/badge.svg)](https://codecov.io/gh/nervosnetwork/ckb-vm)

---

## About CKB VM

CKB VM is a pure software implementation of the [RISC-V](https://riscv.org/) instruction set used as scripting VM in CKB. Right now it implements full IMCB instructions for both 32-bit and 64-bit register size support. In the future we might also implement V extensions to enable better crypto implementations.

## License

Nervos CKB is released under the terms of the MIT license. See [COPYING](COPYING) for more information or see [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT).

## Development Process

This is now deployed and used in production CKB mainnet.

The `develop` branch is regularly built and tested, but is not guaranteed to be completely stable. CKB will use released versions of CKB VM which are tested and more stable.

The contribution workflow is described in [CONTRIBUTING.md](CONTRIBUTING.md), and security policy is described in [SECURITY.md](SECURITY.md). To propose new protocol or standard for Nervos, see [Nervos RFC](https://github.com/nervosnetwork/rfcs).

---

## How to build

CKB VM is currently tested mainly with `stable` Rust version on 64-bit Linux, macOS, and Windows.

```bash
# download CKB VM
$ git clone https://github.com/nervosnetwork/ckb-vm
$ cd ckb-vm
$ cargo build
```

You can also run the tests:

```bash
make test
```

CKB VM has already included RISC-V binaries used in tests, so you don't need a RISC-V compiler to build binaries. However if you do want to play with your own binaries, a RISC-V compiler might be needed. [riscv-tools](https://github.com/riscv/riscv-tools) can be a good starting point here, or if you are an expert on GNU toolchain, you might also compile upstream GCC from source with RISC-V support, [here](./examples/is13.rs) is an example. CKB VM is using standard RISC-V instructions and ELF binary format, so theoretically any RISC-V compatible compilers are able to produce contracts used in CKB VM(tho bug reports are very welcome if you find breakage).

## Notes on Different Modes

Right now CKB VM has 3 different modes:

* Rust interpreter mode
* Assembly based interpreter mode(ASM mode)
* Ahead-of-time compilation mode(AOT mode)

For consistent behavior, you should only use ASM or AOT mode, and it's best if you stick with either ASM or AOT mode depending on your use case. The Rust mode is developed more to assist development, and never used in production by us. In case of bugs, there might be inconsistent behaviors between Rust mode and ASM/AOT mode.
