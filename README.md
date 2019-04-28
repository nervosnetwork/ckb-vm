# [Nervos CKB](https://nervos.org) VM

[![Build Status](https://travis-ci.com/nervosnetwork/ckb-vm.svg?branch=master)](https://travis-ci.com/nervosnetwork/ckb-vm)
[![dependency status](https://deps.rs/repo/github/nervosnetwork/ckb-vm/status.svg)](https://deps.rs/repo/github/nervosnetwork/ckb-vm)

---

## About CKB VM

CKB VM is a pure software implementation of the [RISC-V](https://riscv.org/) instruction set used as scripting VM in CKB. Right now it implements full IMC instructions for both 32-bit and 64-bit register size support. In the future we might also implement V extensions to enable better crypto implementations.

## License

Nervos CKB is released under the terms of the MIT license. See [COPYING](COPYING) for more information or see [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT).

## Development Process

This project is still in development, it's NOT in production ready status.

The `master` branch is regularly built and tested, but is not guaranteed to be completely stable.

The contribution workflow is described in [CONTRIBUTING.md](CONTRIBUTING.md), and security policy is described in [SECURITY.md](SECURITY.md). To propose new protocol or standard for Nervos, see [Nervos RFC](https://github.com/nervosnetwork/rfcs).

---

## How to build

CKB VM is currently tested mainly with `stable-1.32.0` on Linux and macOS.

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

CKB VM has already included RISC-V binaries used in tests, so you don't need a RISC-V compiler to build binaries. However if you do want to play with your own binaries, a RISC-V compiler might be needed. [riscv-tools](https://github.com/riscv/riscv-tools) can be a good starting point here, or if you are an expert on GNU toolchain, you might also compile upstream GCC from source with RISC-V support. CKB VM is using standard RISC-V instructions and ELF binary format, so theoretically any RISC-V compatible compilers are able to produce contracts used in CKB VM(tho bug reports are very welcome if you find breakage).
