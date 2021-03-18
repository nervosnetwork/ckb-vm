riscv64-unknown-elf-as -march=rv64gc -o rvc_pageend.o rvc_pageend.S
riscv64-unknown-elf-ld -o rvc_pageend -T rvc_pageend.lds rvc_pageend.o
rm rvc_pageend.o
riscv64-unknown-elf-objdump -x rvc_pageend > rvc_pageend.dump
