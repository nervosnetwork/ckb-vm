/*
 * First, compile riscv-gnu-toolchain with `--with-arch=rv32imac --with-abi=ilp32`,
 * then compile current file with riscv32-unknown-elf-gcc -o minimal minimal.c
 */
int main(int argc, char* argv[])
{
  if (argc == 1) {
    return 1;
  }
  if (argv[1][0] == 'a') {
    return 2;
  }
  return 0;
}
