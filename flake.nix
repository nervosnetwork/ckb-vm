{
  description = "CKB VM development environment with RISC-V toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        # RISC-V toolchain
        riscv-toolchain = pkgs.pkgsCross.riscv64-embedded.buildPackages;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # RISC-V cross-compiler toolchain
            riscv-toolchain.gcc
            riscv-toolchain.binutils
            
            # Rust toolchain (already in the project via rust-toolchain file)
            cargo
            rustc
            rustfmt
            clippy
            
            # Build tools
            gnumake
            pkg-config
            
            # Optional: useful for development
            gcc
            gdb
            file
            hexdump
            
            # For running the build scripts
            bash
          ];

          shellHook = ''
            echo "CKB VM Development Environment"
            echo "================================"
            echo ""
            echo "RISC-V toolchain available:"
            echo "  riscv64-none-elf-gcc --version"
            riscv64-none-elf-gcc --version | head -n1
            echo ""
            echo "To compile the is13 example:"
            echo "  riscv64-none-elf-gcc -o examples/is13 examples/is13.c"
            echo ""
            echo "To run the example:"
            echo "  cargo run --example is13 13"
            echo ""
          '';

          # Environment variables
          RISCV_CC = "riscv64-none-elf-gcc";
          RISCV_LD = "riscv64-none-elf-ld";
          RISCV_OBJCOPY = "riscv64-none-elf-objcopy";
          RISCV_OBJDUMP = "riscv64-none-elf-objdump";
        };
      });
}